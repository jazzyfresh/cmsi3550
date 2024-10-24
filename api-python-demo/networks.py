from flask import Flask
from flask import jsonify

app = Flask(__name__)

# identify the device we're sending too

@app.route("/api")
def api():
    return "<p>hello api</p>"


@app.route("/other")
def json():
    return {"message":"hello world"}
    # return jsonify({"message":"hello world"})

if __name__ == "__main__":
    app.run(debug=True)








