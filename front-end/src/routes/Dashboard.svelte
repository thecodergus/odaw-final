<script>
  import Chart from "chart.js/auto";
  import { onMount } from "svelte";
  import { get_documentos } from "../services/api";
  import { usuario } from "../services/store";
  import Popup from "../lib/Popup.svelte";
  import {
    calcular_valor_total_mes,
    get_meses_a_partir,
  } from "../services/utils";

  let resumoCanvas;
  let popUpMostrar = false;
  let popUpTitle = "";
  let popUpMensagem = "";

  onMount(async () => {
    // Arrumando datas
    const meses = get_meses_a_partir(new Date(), 2);
    const meses_ptbr = new Intl.DateTimeFormat("pt-BR", { month: "long" });

    let receitas = [];
    let contas = [];
    try {
      const documentos = (await get_documentos(usuario.get().id)).data;
      receitas = documentos.filter((i) => i.tipo_de_documento == "RECEITA");
      contas = documentos.filter((i) => i.tipo_de_documento == "CONTA");
    } catch (err) {
      popUpMostrar = true;
      popUpTitle = "Erro";
      popUpMensagem = "Erro ao buscar documentos";
    }

    const data = {
      labels: meses.map(meses_ptbr.format),
      datasets: [
        {
          label: "Contas",
          data: meses.map((m) => -calcular_valor_total_mes(contas, m)),
          borderColor: "red",
          backgroundColor: "rgba(255, 0, 0, 0.5)",
          borderWidth: 2,
          borderRadius: Number.MAX_VALUE,
          borderSkipped: false,
        },
        {
          label: "Receitas",
          data: meses.map((m) => calcular_valor_total_mes(receitas, m)),
          borderColor: "blue",
          backgroundColor: "rgba(0, 0, 255, 0.5)",
          borderWidth: 2,
          borderRadius: 5,
          borderSkipped: false,
        },
      ],
    };
    const ctx = resumoCanvas.getContext("2d");
    const resumoChart = new Chart(ctx, {
      type: "bar",
      data: data,
      options: {
        responsive: true,
        plugins: {
          legend: {
            position: "top",
          },
          title: {
            display: true,
            text: "",
          },
        },
      },
    });
  });
</script>

<div class="container">
  <h2>Resumo</h2>
  <canvas bind:this={resumoCanvas} id="resumo" width="800px" height="600px"
  ></canvas>
</div>
<Popup
  message={popUpMensagem}
  open={popUpMostrar}
  title={popUpTitle}
  onClosed={() => {
    popUpMostrar = false;
  }}
/>

<style>
</style>
