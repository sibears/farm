from flask import Flask, request, jsonify
import random

app = Flask(__name__)


VALID_TOKEN = "test_token"

@app.route('/flags', methods=['PUT'])
def put_flags():


    
    token = request.headers.get('X-Team-Token')
    if not token or token != VALID_TOKEN:
        return jsonify({"status": False, "msg": f"Invalid token '{token}'"}), 400

    try:
        flags = request.get_json()
        if not isinstance(flags, list):
            raise ValueError
    except:
        return jsonify({"status": False, "msg": "Invalid format"}), 400

    results = []
    for flag in flags:
        
        res = {"status": True, "msg": f"accepted"}

        results.append({"flag": flag, **res})

    return jsonify(results)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=80)