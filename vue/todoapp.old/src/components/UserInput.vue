<template>
  <div id="user-inputs">
    <input v-model="newTodoText" v-on:keydown.enter="createTodo">
    <button v-on:click="clearTodos">
      Clear
    </button>
  </div>
</template>

<script>
 export default {
   data: function () {
     return {newTodoText: ''}
   },
   methods: {
     createTodo: function () {
       this.$store.dispatch('addTodo', {text: this.newTodoText})
       this.newTodoText = ''
       console.log(this.newTodoText, 'created!')
     },
     clearTodos: function () {
       this.$store.dispatch('clearTodos')
       console.log('Todos cleared!')
     }
   }
 }
</script>
