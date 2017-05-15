#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sqlite3
import functools

from bottle import route, run, template
from bottle import jinja2_view, request

view = functools.partial(jinja2_view)

@route('/todo')
@view('make_table.tpl.html')
def todo_list():

    con = sqlite3.connect('todo.db')
    c = con.cursor()
    c.execute("SELECT id, task FROM todo WHERE status LIKE '0'")
    result = c.fetchall()
    c.close()

    return {'result': result}

@route('/new', method = 'GET')
@view('new_task.tpl.html')
def new_item():

    ret = {}

    if request.GET.save:

        new = request.GET.task.strip()

        conn = sqlite3.connect('todo.db')
        c = conn.cursor()

        c.execute("INSERT INTO todo (task, status) VALUES (?, ?)", (new, 1))
        new_id = c.lastrowid

        conn.commit()
        c.close()

        ret.update({'created_task': 'The new task was inserted into the database, the ID is %s' % new_id})

    return ret

if __name__ == '__main__':
    run(port = 8080, debug = True, reloader = True)
