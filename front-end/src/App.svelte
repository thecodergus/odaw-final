<script>
  import { Router, Link, Route, link, navigate } from "svelte-routing";
  import { usuario } from "./services/store";
  import CadastroUsuario from "./routes/CadastroUsuario.svelte";
  import Login from "./routes/Login.svelte";
  import Dashboard from "./routes/Dashboard.svelte";
  import Documento from "./routes/Documento.svelte";

  export let url = "";
  let login = null;

  function logout() {
    usuario.set(null);
    navigate("/login", { replace: true });
    location.reload();
  }
</script>

<Router {url}>
  {#if usuario.get() !== null}
    <nav class="navbar navbar-expand-lg fixed-top nav-cor">
      <div class="container-fluid">
        <a class="navbar-brand" href="/dashboard" use:link replace
          >Sistema de Controle Financeiro Pessoal</a
        >
        <button
          class="navbar-toggler"
          type="button"
          data-bs-toggle="collapse"
          data-bs-target="#navbarNavAltMarkup"
          aria-controls="navbarNavAltMarkup"
          aria-expanded="false"
          aria-label="Toggle navigation"
        >
          <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
          <div class="navbar-nav">
            <a
              class={"nav-link"}
              aria-current="page"
              href="/dashboard"
              use:link
              replace>Dashboard</a
            >
            <a class={"nav-link"} href="/contas" use:link replace
              >Minhas Contas</a
            >
            <a class={"nav-link"} href="/receitas" use:link replace
              >Minhas Receitas</a
            >
            <a class={"nav-link"} href="/perfil" use:link replace>Meu Perfil</a>
            <button
              class="button-link nav-link sair-link"
              type="button"
              on:click={logout}>Sair</button
            >
          </div>
        </div>
      </div>
    </nav>
    <Route path="/dashboard" component={Dashboard} />
    <Route path="/contas" component={Documento} />
    <Route path="*" component={Dashboard} />
  {:else}
    <Route path="/cadastro" component={CadastroUsuario} />
    <Route path="/login" component={Login} />
    <Route path="*" component={Login} />
  {/if}
</Router>

<style>
  .button-link {
    display: inline-block;
    padding: 5px 10px;
    background-color: #4e9caf;
    color: white;
    text-decoration: none;
    border: none;
    border-radius: 5px;
  }

  .nav-cor {
    background-color: #ffc02f;
  }

  .sair-link {
    position: absolute;
    right: 0;
    background-color: #ffc02f;
    color: rgb(38, 46, 58);
  }
</style>
