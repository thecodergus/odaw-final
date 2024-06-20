<script>
  import { Router, Link, Route } from "svelte-routing";

  import CadastroUsuario from "./routes/CadastroUsuario.svelte";
  import Login from "./routes/Login.svelte";
  import Dashboard from "./routes/Dashboard.svelte";

  export let url = "";
</script>

<Router {url}>
  <Route path="/cadastro" component={CadastroUsuario} />
  <Route path="/login" component={Login} />
  <Route path="/dashboard" component={Dashboard} />
</Router>

<style>
</style>
