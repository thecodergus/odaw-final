export  function calcular_valor_total_mes(valores, mes) {
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


  export function get_meses_a_partir(agora, numero_meses){
    let meses = []
    for(let i = -numero_meses; i <= numero_meses; i++){
        if(i !== 0){
            let aux = new Date(agora);
            aux.setMonth(aux.getMonth() + i);
            meses.push(aux)
        }else{
            meses.push(agora)
        }
    }
    return meses;
  }

  export function get_meses_entre(dataInicial, dataFinal) {
  let meses = [];
  let dataAtual = new Date(dataInicial);

  for(let i = 0; dataAtual <= dataFinal; i++){
    dataAtual = new Date(dataInicial);
    dataAtual.setMonth(dataAtual.getMonth() + i);
    meses.push(dataAtual);
  }

  return meses;
}

export function gerar_cores_aleatorias(tamanho) {
  const cores = [];

  for (let i = 0; i < tamanho; i++) {
    const cor = `#${Math.floor(Math.random()*16777215).toString(16)}`;
    cores.push(cor);
  }

  return cores;
}
