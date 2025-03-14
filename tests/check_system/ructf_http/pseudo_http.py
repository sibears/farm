import random

from flask import Flask, jsonify, request

app = Flask(__name__)


@app.route("/flags", methods=["PUT"])
def put_flags():
    token = request.headers.get("X-Team-Token")
    if not token:
        return jsonify({"status": False, "msg": f"Invalid token '{token}'"}), 400

    try:
        flags = request.get_json()
        if not isinstance(flags, list):
            raise ValueError
    except:
        return jsonify({"status": False, "msg": "Invalid format"}), 400

    results = []
    for flag in flags:
        # Генерируем случайный статус для флага
        status_choice = random.choices(
            ["ACCEPTED", "REJECTED", "QUEUED"], weights=[0.5, 0.3, 0.2], k=1
        )[0]

        flag_amount = random.randint(10, 100)

        if status_choice == "ACCEPTED":
            res = {
                "status": True,
                "msg": f"[{flag}] Accepted. {flag_amount} flag points",
            }
        elif status_choice == "REJECTED":
            res = {"status": False, "msg": f"[{flag}] Denied: invalid or own flag"}
        else:  # QUEUED
            res = {"status": False, "msg": "Please try again later"}

        results.append({"flag": flag, **res})

    return jsonify(results)


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8779)
