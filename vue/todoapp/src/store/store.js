import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    todos: [
      { text: 'Learn Vue.' },
      { text: 'Do hard things' },
      { text: 'Lean Django.' }
    ]
  },
  mutations: {
    'ADD_TODO': function (store, todo) {
      store.todos.push(todo)
    },
    'CLEAR_TODOS': function (state) {
      const todos = state.todos
      todos.splice(0, todos.length)
    }
  },
  actions: {
    addTodo (store, todo) {
      store.commit('ADD_TODO', todo)
    },
    clearTodos (store) {
      store.commit('CLEAR_TODOS')
    }
  }
})

export default store
