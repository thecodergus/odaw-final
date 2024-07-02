<script>
  // @ts-nocheck

  // import Chart from "chart.js/auto";
  import { onMount } from "svelte";
  import Tags from "svelte-tags-input";
  import Modal from "../lib/Modal.svelte";
  import Popup from "../lib/Popup.svelte";
  import { gerar_cores_aleatorias, get_meses_entre } from "../services/utils";
  import { Doughnut } from "svelte-chartjs";

  import { DateInput } from "date-picker-svelte";
  import {
    deletar_documento,
    get_categorias,
    get_documentos,
    atualizar_documento,
    cadastrar_documento,
    cadastrar_categoria,
    deletar_categoria,
  } from "../services/api";
  import { usuario } from "../services/store";
  import { each } from "chart.js/helpers";

  export let tipo_documento = "";
  export let titulo_pagina = "";

  // Datas
  let data_inicio = new Date();
  data_inicio.setMonth(data_inicio.getMonth() - 6);
  let data_fim = new Date();
  data_fim.setMonth(data_fim.getMonth() + 6);
  const meses_ptbr = new Intl.DateTimeFormat("pt-BR", { month: "long" });

  // Tags
  let tags = [];

  // Lista de Documentos
  let documentos = [];

  // Canvas
  let chartRef;
  let chartData = {};

  // Popup
  let popUpMostrar = false;
  let popUpTitulo = "";
  let popUpMensagem;

  // Modais
  let modalCriarDocumento = {
    mostrar: false,
    titulo: `Criar ${tipo_documento.toLowerCase()}`,
    aoFechar: () => {
      modalCriarDocumento.mostrar = false;
    },
    aoSalvar: async () => {
      try {
        await cadastrar_documento(
          tipo_documento,
          modalCriarDocumento.data.descricao,
          modalCriarDocumento.data.valor,
          modalCriarDocumento.data.data.toISOString().split("T")[0],
          modalCriarDocumento.data.id_usuario,
          modalCriarDocumento.data.categorias
        );
      } catch (err) {
        modalCriarDocumento.mostrar = false;
        popUpTitulo = "Erro";
        popUpMensagem = `Erro ao criar documento: ${err}`;
        popUpMostrar = true;
      }
      modalCriarDocumento.mostrar = false;
      atualizar_tudo();
    },
    data: {
      descricao: "",
      valor: 0,
      data: new Date(),
      id_usuario: usuario.get().id,
      categorias: [],
    },
  };

  let modalEditarDocumento = {
    mostrar: false,
    titulo: `Editar ${tipo_documento.toLowerCase()}`,
    aoFechar: () => {
      modalEditarDocumento.mostrar = false;
    },
    aoSalvar: async () => {
      try {
        await atualizar_documento(
          modalEditarDocumento.data.id,
          modalEditarDocumento.data.descricao,
          modalEditarDocumento.data.valor,
          modalEditarDocumento.data.data.toISOString().split("T")[0],
          modalEditarDocumento.data.id_usuario
        );

        await Promise.all(
          modalEditarDocumento.data.categorias.map(async (cat) => {
            await cadastrar_categoria(modalEditarDocumento.data.id, cat);
          })
        );
      } catch (err) {
        modalEditarDocumento.mostrar = false;
        popUpTitulo = "Erro";
        popUpMensagem = `Erro ao editar documento: ${err}`;
        popUpMostrar = true;
      }
      modalEditarDocumento.mostrar = false;
      atualizar_tudo();
    },
    data: {
      descricao: "",
      valor: 0,
      data: new Date(),
      id_usuario: usuario.get().id,
      categorias: [],
    },
  };

  let modalExcluirDocumento = {
    mostrar: false,
    titulo: `Excluir ${tipo_documento.toLowerCase()}`,
    aoFechar: () => {
      modalExcluirDocumento.mostrar = false;
    },
    aoSalvar: async () => {
      try {
        await deletar_documento(modalExcluirDocumento.data.id);
      } catch (err) {
        modalEditarDocumento.mostrar = false;
        popUpTitulo = "Erro";
        popUpMensagem = `Erro ao excluir documento: ${err}`;
        popUpMostrar = true;
      }
      atualizar_tudo();
      modalExcluirDocumento.mostrar = false;
    },
    data: {
      descricao: "",
      valor: 0,
      data: new Date(),
      id_usuario: usuario.get().id,
      categorias: [],
    },
  };

  let modalVerDocumento = {
    mostrar: false,
    titulo: `Ver ${tipo_documento.toLowerCase()}`,
    aoFechar: () => {
      modalVerDocumento.mostrar = false;
    },
    aoSalvar: () => {},
  };

  // Canvas
  let resumoChart = null;

  // Funções
  async function editar_documento(documento) {
    modalEditarDocumento.mostrar = true;
    modalEditarDocumento.data = documento;
    modalEditarDocumento.data.categorias = documento.categorias.map(
      (i) => i.nome
    );
  }

  async function excluir_documento(documento) {
    modalExcluirDocumento.mostrar = true;
    modalExcluirDocumento.data = documento;
  }

  async function ver_documento(documento) {
    modalVerDocumento.mostrar = true;
    modalVerDocumento.data = documento;
  }

  async function criar_documento() {
    modalCriarDocumento.mostrar = true;
    modalCriarDocumento.data = {
      descricao: "",
      valor: 0,
      data: new Date(),
      id_usuario: usuario.get().id,
      categorias: [],
    };
  }

  function escrever_data_cursivamente(data) {
    const opcoes = {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    };
    const aux = new Intl.DateTimeFormat("pt-BR", opcoes);

    return aux.format(new Date(data));
  }

  async function atualizar_documentos() {
    try {
      const res = await get_documentos(usuario.get().id);

      // Filtrando querendo apenas receitas ou contas e buscando as categorias de cada
      documentos = await Promise.all(
        res.data
          .filter((i) => {
            const atual = new Date(i.data);
            const inicio = new Date(data_inicio);
            const fim = new Date(data_fim);

            return (
              i.tipo_de_documento === tipo_documento &&
              atual >= inicio &&
              atual <= fim
            );
          })
          .map(async (i) => {
            try {
              const categorias = await get_categorias(i.id);
              i.categorias = categorias.data;
              i.data = new Date(i.data);
            } catch (e) {
              popUpTitulo = "Erro";
              popUpMensagem = `Erro ao buscar categorias: ${e}`;
              popUpMostrar = true;
            }

            return i;
          })
      );
      // Filtrar pela questão categorias
      if (tags.length > 0) {
        documentos = documentos.filter((i) =>
          i.categorias.some((i_) =>
            tags.some((i__) => i_.nome.toLowerCase() === i__.toLowerCase())
          )
        );
      }
    } catch (err) {
      popUpTitulo = "Erro";
      popUpMensagem = `Erro ao buscar documentos: ${err}`;
      popUpMostrar = true;
    }
  }

  // Canvas

  function atualizar_grafico() {
    if (chartRef !== null) {
      const meses_label = get_meses_entre(data_inicio, data_fim);
      chartData = {
        labels: meses_label.map(meses_ptbr.format),
        datasets: [
          {
            data: meses_label.map((mes) => {
              return documentos.filter((doc) => {
                const data = new Date(doc.data);
                return (
                  data.getMonth() === mes.getMonth() &&
                  data.getFullYear() === mes.getFullYear()
                );
              }).length;
            }),
          },
        ],
      };

      chartRef.update();
    }
  }

  async function atualizar_tudo() {
    await atualizar_documentos();
    await atualizar_grafico();
  }

  //Start
  onMount(async () => {
    await atualizar_tudo();
  });

  async function remover_categoria(id_documento) {
    try {
      const response = await get_categorias(id_documento);
      const categoria = response.data.find(
        (i) => !modalEditarDocumento.data.categorias.includes(i.nome)
      );

      if (categoria) {
        await deletar_categoria(categoria.id);
      }
    } catch (err) {
      popUpTitulo = "Erro";
      popUpMensagem = `Erro ao deletar categoria: ${err}`;
      popUpMostrar = true;
    }
  }
</script>

<div class="container">
  <div class="row">
    <h2>Minhas {titulo_pagina}</h2>
    <button class="btn btn-primary botao-amarelo" on:click={criar_documento}
      >Criar {tipo_documento.toLowerCase()}</button
    >
  </div>
  <div class="row">
    <div class="col">
      <label for="tags">Categorias</label>
      <Tags
        id={"tags"}
        bind:tags
        onTagAdded={atualizar_tudo}
        onTagRemoved={atualizar_tudo}
      />
    </div>
    <div class="col-6">
      <Doughnut
        data={chartData}
        options={{ responsive: true }}
        bind:chart={chartRef}
      />
    </div>
    <div class="col">
      <div class="row">
        <label for="data-inicio">Data Inicio</label>
        <DateInput
          id={"data-inicio"}
          bind:value={data_inicio}
          format={"dd/MM/yyyy"}
          closeOnSelection={true}
          on:select={atualizar_tudo}
        />
      </div>
      <div class="row">
        <label for="data-fim">Data Fim</label>
        <DateInput
          id={"data-fim"}
          bind:value={data_fim}
          format={"dd/MM/yyyy"}
          closeOnSelection={true}
          on:select={atualizar_tudo}
        />
      </div>
    </div>
  </div>
  <div class="row">
    <table class="table table-striped">
      <thead>
        <tr>
          <th scope="col">Nome da Conta</th>
          <th scope="col">Categorias</th>
          <th scope="col">Opções</th>
        </tr>
      </thead>
      <tbody>
        {#each documentos as doc}
          <tr>
            <th scope="row">{doc.descricao}</th>
            <td>
              {#each doc.categorias as cat}
                <span class="badge bg-secondary">{cat.nome}</span>
              {/each}
            </td>
            <td>
              <button class="btn btn-opcoes" on:click={() => ver_documento(doc)}
                >Ver</button
              >
              <button
                class="btn btn-opcoes"
                on:click={() => editar_documento(doc)}>Editar</button
              >
              <button
                class="btn btn-opcoes"
                on:click={() => excluir_documento(doc)}>Excluir</button
              >
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<Popup
  message={popUpMensagem}
  onClosed={() => (popUpMostrar = false)}
  open={popUpMostrar}
  title={popUpTitulo}
/>

<Modal
  onClosed={modalCriarDocumento.aoFechar}
  open={modalCriarDocumento.mostrar}
  title={modalCriarDocumento.titulo}
  onSave={modalCriarDocumento.aoSalvar}
>
  {#if modalCriarDocumento.data}
    <div class="form-control">
      <div class="mb-3">
        <div class="input-group">
          <span class="input-group-text" id="basic-addon3">Descrição:</span>
          <input
            type="text"
            class="form-control"
            id="basic-url"
            aria-describedby="basic-addon3 basic-addon4"
            bind:value={modalCriarDocumento.data.descricao}
          />
        </div>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">R$</span>
        <input
          type="text"
          class="form-control"
          aria-label="Amount (to the nearest dollar)"
          bind:value={modalCriarDocumento.data.valor}
        />
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Data</span>
        <DateInput
          id={"data-inicio"}
          bind:value={modalCriarDocumento.data.data}
          format={"dd/MM/yyyy"}
          closeOnSelection={true}
          on:select={atualizar_tudo}
        />
      </div>
      <!-- <div class="input-group mb-3">
        <span class="input-group-text">Categorias</span>
        <div class="input-group">
          {#each modalCriarDocumento.data.categorias as cat}
            <span class="badge bg-secondary">{cat.nome}</span>
          {/each}
        </div>
      </div> -->
    </div>
  {/if}
</Modal>

<Modal
  onClosed={modalVerDocumento.aoFechar}
  open={modalVerDocumento.mostrar}
  title={modalVerDocumento.titulo}
  onSave={modalVerDocumento.aoSalvar}
  showSalvar={false}
>
  {#if modalVerDocumento.data}
    <div class="form-control">
      <div class="mb-3">
        <div class="input-group">
          <span class="input-group-text" id="basic-addon3">Descrição:</span>
          <input
            type="text"
            class="form-control"
            id="basic-url"
            aria-describedby="basic-addon3 basic-addon4"
            value={modalVerDocumento.data.descricao}
            disabled
          />
        </div>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">R$</span>
        <input
          type="text"
          class="form-control"
          aria-label="Amount (to the nearest dollar)"
          value={modalVerDocumento.data.valor}
          disabled
        />
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Data</span>
        <input
          type="text"
          class="form-control"
          aria-label="Amount (to the nearest dollar)"
          value={escrever_data_cursivamente(modalVerDocumento.data.data)}
          disabled
        />
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Categorias</span>
        <div class="input-group">
          {#each modalVerDocumento.data.categorias as cat}
            <span class="badge bg-secondary">{cat.nome}</span>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</Modal>

<Modal
  onClosed={modalEditarDocumento.aoFechar}
  open={modalEditarDocumento.mostrar}
  title={modalEditarDocumento.titulo}
  onSave={modalEditarDocumento.aoSalvar}
>
  {#if modalEditarDocumento.data}
    <div class="form-control">
      <div class="mb-3">
        <div class="input-group">
          <span class="input-group-text" id="basic-addon3">Descrição:</span>
          <input
            type="text"
            class="form-control"
            id="basic-url"
            aria-describedby="basic-addon3 basic-addon4"
            bind:value={modalEditarDocumento.data.descricao}
          />
        </div>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">R$</span>
        <input
          type="text"
          class="form-control"
          aria-label="Amount (to the nearest dollar)"
          bind:value={modalEditarDocumento.data.valor}
        />
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Data</span>
        <DateInput
          id={"data-inicio"}
          bind:value={modalEditarDocumento.data.data}
          format={"dd/MM/yyyy"}
          closeOnSelection={true}
          on:select={atualizar_tudo}
        />
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Categorias</span>
        <Tags
          id={"tag-input"}
          bind:tags={modalEditarDocumento.data.categorias}
          onTagRemoved={() => remover_categoria(modalEditarDocumento.data.id)}
        />
      </div>
    </div>
  {/if}
</Modal>

<Modal
  onClosed={modalExcluirDocumento.aoFechar}
  open={modalExcluirDocumento.mostrar}
  title={modalExcluirDocumento.titulo}
  onSave={modalExcluirDocumento.aoSalvar}
>
  <h1>Tem certeza que deseja excluir? Clique em salvar</h1>
</Modal>

<style>
  div .row {
    margin-top: 10px;
  }
  .btn-opcoes {
    background: none !important;
    border: none;
    padding: 0 !important;
    font-family: Arial, sans-serif;
    color: #069;
    text-decoration: underline;
    cursor: pointer;
    margin-left: 5px;
  }
  #tag-input {
    color: black !important;
  }
  #tags {
    color: black !important;
  }
</style>
