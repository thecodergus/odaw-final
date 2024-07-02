<script>
  // Importe a variável reativa do Svelte
  import Popup from "../lib/Popup.svelte";
  import { onMount } from "svelte";
  import { validarEmail } from "../utils";
  import {
    atualizar_usuario,
    cadastrar_usuario,
    fazer_login,
    get_usuario,
  } from "../services/api";
  import { DateInput } from "date-picker-svelte";
  import { navigate } from "svelte-routing";
  import { usuario } from "../services/store";

  // Crie variáveis reativas para os campos
  let usu = usuario.get();
  let nome = "";
  let email = "";
  let senha = "";
  let data_de_nascimento = new Date(usu.data_de_nascimento);
  let confirmarSenha = "";
  let popupVisivel = false;
  let popupMensagem = "";
  let popupCallBack = null;
  let popUpTitulo = "Erro";

  // Função para verificar se as senhas coincidem
  function verificarCampos() {
    if (senha !== confirmarSenha) {
      abrirPopup("As senhas não coincidem. Por favor, tente novamente.");
    } else if ([nome, email, senha, confirmarSenha].some((i) => i === "")) {
      abrirPopup("Os campos não podem ficar em branco!");
    } else if (validarEmail(email) === false) {
      abrirPopup("O e-mail precisa ser valido!");
    } else {
      atualizar_usuario(
        usu.id,
        nome,
        senha,
        email,
        data_de_nascimento.toISOString().split("T")[0]
      )
        .then(() => {
          popUpTitulo = "Sucesso";
          abrirPopup("Usuario atualizado com sucesso!");
          senha = "";
          confirmarSenha = "";
          atualizar_informacoes();
        })
        .catch((err) => {
          senha = "";
          confirmarSenha = "";
          popUpTitulo = "Erro";
          abrirPopup(err.response.data.mensagem);
        });
    }
  }

  async function atualizar_informacoes() {
    try {
      const response = await get_usuario(usu.id);
      usuario.set(response.data);
      nome = response.data.nome;
      email = response.data.email;
      data_de_nascimento = new Date(response.data.data_de_nascimento);
    } catch (err) {
      popUpTitulo = "Erro";
      abrirPopup(err.response.data.mensagem);
    }
  }

  function abrirPopup(msg) {
    popupMensagem = msg;
    popupCallBack = () => {
      popupVisivel = false;
      popupMensagem = "";
    };
    popupVisivel = true;
  }

  onMount(() => {
    atualizar_informacoes();
  });
</script>

<div class="container">
  <form>
    <div class="row">
      <h2>Atualizar Usuario</h2>
    </div>
    <div class="mb-3">
      <div class="row">
        <label for="cadastroNome" class="form-label">Nome</label>
      </div>
      <div class="row">
        <input
          type="text"
          class="form-control"
          id="cadastroNome"
          bind:value={nome}
        />
      </div>
    </div>
    <div class="mb-3">
      <div class="row">
        <label for="cadastroEmail" class="form-label">E-mail</label>
      </div>
      <div class="row">
        <input
          type="email"
          class="form-control"
          id="cadastroEmail"
          aria-describedby="emailHelp"
          bind:value={email}
        />
      </div>
    </div>
    <div class="mb-3">
      <div class="row">
        <label for="cadastroData" class="form-label">Data de Nascimento</label>
      </div>
      <div class="row">
        <DateInput
          id={"cadastroData"}
          bind:value={data_de_nascimento}
          format={"dd-MM-yyyy"}
          min={new Date(1900, 0, 1)}
          closeOnSelection={true}
        />
      </div>
    </div>
    <div class="mb-3">
      <div class="row">
        <label for="cadastroSenha" class="form-label">Password</label>
      </div>
      <div class="row">
        <input
          type="password"
          class="form-control"
          id="cadastroSenha"
          bind:value={senha}
        />
      </div>
    </div>
    <div class="mb-3">
      <div class="row">
        <label for="cadastroConfirmarSenha" class="form-label"
          >Confirmar Senha</label
        >
      </div>
      <div class="row">
        <input
          type="password"
          class="form-control"
          id="cadastroConfirmarSenha"
          bind:value={confirmarSenha}
        />
      </div>
    </div>
    <div class="row">
      <button
        type="button"
        class="btn btn-warning botao-amarelo"
        on:click={verificarCampos}>Atualizar conta de usuario</button
      >
    </div>
  </form>
</div>
<Popup
  open={popupVisivel}
  message={popupMensagem}
  title={popUpTitulo}
  onClosed={popupCallBack}
/>

<style></style>
