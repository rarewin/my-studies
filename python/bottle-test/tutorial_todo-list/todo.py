#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sqlite3
from bottle import route, run

@route('/todo')
def todo_list():

    con = sqlite3.connect('todo.db')
    c = con.cursor()
    c.execute("SELECT id, task FROM todo WHERE status LIKE '0'")
    result = c.fetchall()

    return str(result)

run(port = 8080, debug = True, reloader = True)
