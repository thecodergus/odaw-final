<script>
    import Popup from "../lib/Popup.svelte";
    import { validarEmail } from "../utils";

    // Campos proprios
    let email = "";
    let senha = "";

    // Campos do popup
    let showPopup = false;
    let titlePopup = "Erro";
    let messagePopup = "";
    let callbackPopup = () => {
        showPopup = false;
    };

    function abrirPopup(msg) {
        messagePopup = msg;
        showPopup = true;
    }

    function verificarCampos() {
        if ([email, senha].some((i) => i === "")) {
            abrirPopup("Nenhum campo pode estar vazio");
        }

        if (validarEmail(email) === false) {
            abrirPopup("O e-mail precisar ser valido!");
        }
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
            <label for="email" class="form-label">E-mail</label>
        </div>
        <div class="row">
            <input
                type="email"
                class="form-control"
                id="email"
                aria-describedby="emailHelp"
                bind:value={email}
            />
        </div>
        <div class="row">
            <label for="password" class="form-label">Senha</label>
        </div>
        <div class="row">
            <input
                type="password"
                class="form-control"
                id="cadastroSenha"
                bind:value={senha}
            />
        </div>

        <div class="row">
            <button
                type="button"
                class="btn btn-warning botao-amarelo"
                on:click={verificarCampos}>Fazer Login</button
            >
        </div>
        <div class="row">
            <button type="button" class="btn btn-primary botao-preto"
                >Registrar-se</button
            >
        </div>
    </form>
</div>
<Popup
    open={showPopup}
    message={messagePopup}
    onClosed={callbackPopup}
    title={titlePopup}
/>

<style>
    div .row {
        margin-top: 10px;
    }
</style>
