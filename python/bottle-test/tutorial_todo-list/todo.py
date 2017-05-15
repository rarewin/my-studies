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

        con = sqlite3.connect('todo.db')
        c = con.cursor()

        c.execute("INSERT INTO todo (task, status) VALUES (?, ?)", (new, 1))
        new_id = c.lastrowid

        con.commit()
        c.close()

        ret.update({'created_task': 'The new task was inserted into the database, the ID is %s' % new_id})

    return ret

@route('/edit/<no:int>', method = 'GET')
@view('edit_task.tpl.html')
def edit_item(no):

    if request.GET.save:

        edit   = request.GET.task.strip()
        status = request.GET.status.strip()

        status = 1 if status == 'open' else 0

        con = sqlite3.connect('todo.db')
        c = con.cursor()
        c.execute("UPDATE todo SET task = ?, status = ? WHERE id LIKE ?", (edit, status, no))
        con.commit()
        cur_data = [edit,]

    else:

        con = sqlite3.connect('todo.db')
        c = con.cursor()
        c.execute("SELECT task FROM todo WHERE id LIKE ?", (str(no)))
        cur_data = c.fetchone()

    return {'no':  no,
            'old': cur_data}


if __name__ == '__main__':
    run(port = 8080, debug = True, reloader = True)
