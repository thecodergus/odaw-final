<script>
  import { Router, Link, Route } from "svelte-routing";

  import CadastroUsuario from "./routes/CadastroUsuario.svelte";

  export let url = "";
</script>

<Router {url}>
  <Route path="/cadastro" component={CadastroUsuario} />
</Router>

<style>
</style>
