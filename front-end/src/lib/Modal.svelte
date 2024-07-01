<script>
  export let open = false;
  export let title = "";
  export let showBackdrop = true;
  export let onClosed;
  export let onSave;
  export let showSalvar = true;

  const modalClose = () => {
    open = false;
    if (onClosed) {
      onClosed();
    }
  };
</script>

{#if open}
  <div
    class="modal"
    id="sampleModal"
    tabindex="-1"
    role="dialog"
    aria-labelledby="sampleModalLabel"
    aria-hidden={false}
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title" id="sampleModalLabel">
            {title}
          </h5>
        </div>
        <div class="modal-body">
          <slot></slot>
        </div>
        <div class="modal-footer">
          <button
            type="button"
            class="btn btn-danger botao-preto"
            data-dismiss="modal"
            on:click={modalClose}>Fechar</button
          >
          {#if showSalvar}
            <button
              type="button"
              class="btn btn-primary botao-amarelo"
              data-dismiss="modal"
              on:click={onSave}>Salvar</button
            >
          {/if}
        </div>
      </div>
    </div>
  </div>
  {#if showBackdrop}
    <div class="modal-backdrop show" />
  {/if}
{/if}

<style>
  .modal {
    display: block;
  }
</style>
