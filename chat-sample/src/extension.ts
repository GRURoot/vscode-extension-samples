import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(registerChatTool());
    context.subscriptions.push(registerChatParticipant());
}

function registerChatTool() {
    return vscode.lm.registerTool('chat-sample.activeTabCount', {
        async invoke(parameters, token) {
            return {
                toString() {
                    const activeTabCount = vscode.window.tabGroups.activeTabGroup.tabs.length;
                    return `There are ${activeTabCount} tabs open.`;
                },
            };
        },
    });
}

function registerChatParticipant() {
    const handler: vscode.ChatRequestHandler = async (request: vscode.ChatRequest, chatContext: vscode.ChatContext, stream: vscode.ChatResponseStream, token: vscode.CancellationToken) => {
        const models = await vscode.lm.selectChatModels({
            vendor: 'copilot',
            family: 'gpt-4o'
        });

        const model = models[0];
        stream.markdown(`Available tools: ${vscode.lm.tools.map(tool => tool.name).join(', ')}\n\n`);

        const allTools = vscode.lm.tools.map((tool): vscode.LanguageModelChatFunction => {
            return {
                name: tool.name.replace(/\./g, '_'),
                description: tool.description,
                parametersSchema: tool.parametersSchema ?? {}
            };
        });

        const options: vscode.LanguageModelChatRequestOptions = {
            justification: 'Just because!',
        };

        const messages = [
            vscode.LanguageModelChatMessage.User(`There is a selection of tools that may give helpful context to answer the user's query. If you aren't sure which tool is relevant, you can call multiple tools.`),
            vscode.LanguageModelChatMessage.User(request.prompt),
        ];
        const runWithFunctions = async () => {
            const requestedTool = request.requestedTools?.shift();
            if (requestedTool) {
                options.toolChoice = requestedTool;
                options.tools = allTools.filter(tool => tool.name === requestedTool);
            } else {
                options.toolChoice = undefined;
                options.tools = allTools;
            }

            let didReceiveFunctionUse = false;

            const response = await model.sendRequest(messages, options, token);

            for await (const part of response.stream) {
                if (part instanceof vscode.LanguageModelChatResponseTextPart) {
                    stream.markdown(part.value);
                } else if (part instanceof vscode.LanguageModelChatResponseFunctionUsePart) {
                    const tool = vscode.lm.tools.find(tool => tool.name.replace(/\./g, '_') === part.name);
                    if (!tool) {
                        // BAD tool choice?
                        continue;
                    }

                    let parameters: any;
                    try {
                        parameters = JSON.parse(part.parameters);
                    } catch (err) {
                        throw new Error(`Got invalid tool use parameters: "${part.parameters}". (${(err as Error).message})`);
                    }

                    const resultPromise = vscode.lm.invokeTool(tool.name, JSON.parse(part.parameters), token);
                    stream.progress(`FUNCTION_CALL: ${tool.name} with ${part.parameters}`, async () => {
                        await resultPromise;
                    });

                    const result = await resultPromise;

                    // NOTE that the result of calling a function is a special content type of a USER-message
                    let message = vscode.LanguageModelChatMessage.User('');
                    message.content2 = new vscode.LanguageModelChatMessageFunctionResultPart(tool.name.replace(/\./g, '_'), result.toString());
                    messages.push(message);

                    // IMPORTANT 
                    // IMPORTANT working around CAPI always wanting to end with a `User`-message
                    // IMPORTANT 
                    messages.push(vscode.LanguageModelChatMessage.User(`Above is the result of calling the function ${tool.name}. The user cannot see this result, so you should explain it to the user if referencing it in your answer.`));
                    didReceiveFunctionUse = true;
                }
            }

            if (didReceiveFunctionUse) {
                // RE-enter
                return runWithFunctions();
            }
        };

        await runWithFunctions();
    };

    const toolUser = vscode.chat.createChatParticipant('chat-sample.tools', handler);
    toolUser.iconPath = new vscode.ThemeIcon('tools');

    return toolUser;
}


export function deactivate() { }
