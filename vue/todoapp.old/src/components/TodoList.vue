<template>
  <div id="todolist">
    <ul>
      <li v-for="todo in todos">
	<todo v-bind:todo="todo"
              v-bind:id="todo.id">
	</todo>
      </li>
    </ul>
  </div>
</template>

<script>
 import Todo from './Todo.vue'

 export default {
   components: {
     Todo
   },
   computed: {
     todos () {
       return this.$store.state.todos
     }
   }
 }
</script>
