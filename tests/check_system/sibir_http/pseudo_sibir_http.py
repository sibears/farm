import random

from flask import Flask, jsonify, request

app = Flask(__name__)


@app.route("/flag", methods=["GET"])
def get_flag():
    team_id = request.args.get("teamid")
    flag = request.args.get("flag")
    
    if not team_id or not flag:
        return "Invalid parameters", 400
    
    status_choice = random.choices([200, 403, 400], weights=[0.5, 0.3, 0.2], k=1)[0]
    
    if status_choice == 200:
        return f"OK (flag accepted) {flag}", 200
    elif status_choice == 403:
        return f"FAIL {flag} Flag not accepted", 403
    elif status_choice == 400:
        return f"FAIL Request incorrect. {flag}", 400
    else:
        return f"FAIL Something went wrong. {status_choice}", 500


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8779)
