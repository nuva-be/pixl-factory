import json
import os
import sys

SESSION_ID = "sess-1"


def send(message):
    print(json.dumps(message), flush=True)


def respond(message, result):
    send({"jsonrpc": "2.0", "id": message["id"], "result": result})


for line in sys.stdin:
    message = json.loads(line)
    method = message.get("method")
    if method == "initialize":
        respond(message, {"protocolVersion": 1, "agentCapabilities": {}})
    elif method == "session/new":
        respond(message, {"sessionId": SESSION_ID})
    elif method == "session/prompt":
        with open("hello.txt", "w", encoding="utf-8") as file:
            file.write("hello from acp\n")
        for text in ["hello ", "from acp"]:
            send({
                "jsonrpc": "2.0",
                "method": "session/update",
                "params": {
                    "sessionId": SESSION_ID,
                    "update": {
                        "sessionUpdate": "agent_message_chunk",
                        "content": {"type": "text", "text": text},
                    },
                },
            })
        respond(message, {"stopReason": os.environ.get("ACP_STOP_REASON", "end_turn")})
        break
