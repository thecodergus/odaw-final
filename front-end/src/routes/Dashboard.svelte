<script>
  import Chart from "chart.js/auto";
  import { onMount } from "svelte";
  import { get_documentos } from "../services/api";
  import { usuario } from "../services/store";
  import Popup from "../lib/Popup.svelte";

  let resumoCanvas;
  let popUpMostrar = false;
  let popUpTitle = "";
  let popUpMensagem = "";

  function calcular_valor_total_mes(valores, mes) {
    const mes_atual = mes.getMonth();
    return valores.reduce((acc, v) => {
      try {
        const data = new Date(v.data);
        const valor = v.valor;
        if (data.getMonth() === mes_atual) {
          return acc + parseFloat(valor);
        }
      } catch (err) {
        return acc;
      }
      return acc;
    }, 0);
  }

  onMount(async () => {
    // Arrumando datas
    const data_atual = new Date();
    const anterior_mes_2 = new Date(data_atual);
    anterior_mes_2.setMonth(anterior_mes_2.getMonth() - 2);
    const anterior_mes_1 = new Date(data_atual);
    anterior_mes_1.setMonth(anterior_mes_1.getMonth() - 1);
    const proximo_mes_1 = new Date(data_atual);
    proximo_mes_1.setMonth(proximo_mes_1.getMonth() + 1);
    const proximo_mes_2 = new Date(data_atual);
    proximo_mes_2.setMonth(proximo_mes_2.getMonth() + 2);
    const meses = new Intl.DateTimeFormat("pt-BR", { month: "long" });

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
      labels: [
        meses.format(anterior_mes_2),
        meses.format(anterior_mes_1),
        meses.format(data_atual),
        meses.format(proximo_mes_1),
        meses.format(proximo_mes_2),
      ],
      datasets: [
        {
          label: "Contas",
          data: [
            -calcular_valor_total_mes(contas, anterior_mes_2),
            -calcular_valor_total_mes(contas, anterior_mes_1),
            -calcular_valor_total_mes(contas, data_atual),
            -calcular_valor_total_mes(contas, proximo_mes_1),
            -calcular_valor_total_mes(contas, proximo_mes_2),
          ],
          borderColor: "red",
          backgroundColor: "rgba(255, 0, 0, 0.5)",
          borderWidth: 2,
          borderRadius: Number.MAX_VALUE,
          borderSkipped: false,
        },
        {
          label: "Receitas",
          data: [
            calcular_valor_total_mes(receitas, anterior_mes_2),
            calcular_valor_total_mes(receitas, anterior_mes_1),
            calcular_valor_total_mes(receitas, data_atual),
            calcular_valor_total_mes(receitas, proximo_mes_1),
            calcular_valor_total_mes(receitas, proximo_mes_2),
          ],
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
            text: "Chart.js Bar Chart",
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
