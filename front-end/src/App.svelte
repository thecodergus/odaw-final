<script>
  import { Router, Link, Route } from "svelte-routing";

  import CadastroUsuario from "./routes/CadastroUsuario.svelte";
  import Login from "./routes/Login.svelte";

  export let url = "";
</script>

<Router {url}>
  <Route path="/cadastro" component={CadastroUsuario} />
  <Route path="/login" component={Login} />
</Router>

<style>
</style>
