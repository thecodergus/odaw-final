import { API_URL } from "../config";
import axios from "axios";

const headers = {
    'Content-Type': 'application/json'
}

async function get(caminho) {
    return await axios.get(`${API_URL}/${caminho}`, { headers: headers })
}

async function post(caminho, parametros) {
    return await axios.post(`${API_URL}/${caminho}/`, parametros, {
        headers: headers
    });
}
async function put(caminho, parametros) {
    return await axios.put(`${API_URL}/${caminho}/`, parametros, {
        headers: headers
    });
}
async function delete_(caminho, parametros) {
    return await axios.delete(`${API_URL}/${caminho}/`, { headers: headers });
}

export async function fazer_login(email, senha) {
    return post("login", {
        email,
        senha
    })
}

export async function cadastrar_usuario(nome, senha, email, data_de_nascimento) {
    return post("usuario", {
        nome,
        senha,
        email,
        data_de_nascimento
    })
}

export async function get_documentos(id_usuario) {
    return get(`documento/usuario/${id_usuario}`)
}