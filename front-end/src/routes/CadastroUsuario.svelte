<script>
    // Importe a variável reativa do Svelte
    import Popup from "../lib/Popup.svelte";
    import { validarEmail } from "../utils";

    // Crie variáveis reativas para os campos
    let nome = "";
    let email = "";
    let senha = "";
    let confirmarSenha = "";
    let popupVisivel = false;
    let popupMensagem = "";
    let popupCallBack = null;

    // Função para verificar se as senhas coincidem
    function verificarCampos() {
        if (senha !== confirmarSenha) {
            abrirPopup("As senhas não coincidem. Por favor, tente novamente.");
        }

        if ([nome, email, senha, confirmarSenha].some((i) => i === "")) {
            abrirPopup("Os campos não podem ficar em branco!");
        }

        if (validarEmail(email) === false) {
            abrirPopup("O e-mail precisa ser valido!");
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
</script>

<div class="container">
    <picture>
        <img
            src="/imgs/Logo.jpeg"
            class="img-fluid img-thumbnail logo"
            alt="Imagem com formatos diferentes"
        />
    </picture>

    <form>
        <div class="row">
            <h2>Cadastro de Usuario</h2>
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
                on:click={verificarCampos}>Registrar conta de usuario</button
            >
        </div>
    </form>
</div>
<Popup
    open={popupVisivel}
    message={popupMensagem}
    title={"Erro"}
    onClosed={popupCallBack}
/>

<style></style>
