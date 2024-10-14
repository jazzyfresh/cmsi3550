from flask import Flask

app = Flask(__name__)

count_num = [0]
# count_num = 0

print(type(count_num))

@app.route("/")
def hello_world():
    return "Hello, world"

@app.route("/count")
def count():
    count_num[0] += 1
    return f"Count is {count_num}"

@app.route("/json/<username>")
def json(username):
    persons = { "loosh" : "Lucian Prinz",
               "MearnsAaron": "Aaron Mearns",
               "gabriel-twiggho": "Gabe Twiggho",
               "Milo-Coding": "Milo Fritzen",
               "jazzyfresh" : "Jasmine Dahilig" }
    return f"{persons}"
