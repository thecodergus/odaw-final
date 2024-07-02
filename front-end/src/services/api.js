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
async function delete_(caminho) {
    return await axios.delete(`${API_URL}/${caminho}`);
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

export async function get_categorias(id_documento){
    return get(`categoria/documento/${id_documento}`)
}

export async function deletar_documento(id_documento){
    return delete_(`documento/${id_documento}`)
}


export async function atualizar_documento(id_documento, descricao, valor, data, id_usuario){
    return put(`documento`, {
        id: id_documento,
        descricao,
        valor,
        data,
        id_usuario        
    })
}

export async function cadastrar_documento(tipo_de_documento, descricao, valor, data, id_usuario, categorias){
    return post(`documento`, {
        tipo_de_documento,
        descricao,
        valor,
        data,
        id_usuario        
    })
}   

export async function atualizar_usuario(id_usuario, nome, senha, email, data_de_nascimento){
    return put(`usuario`, {
        id: id_usuario,
        nome,
        senha,
        email,
        data_de_nascimento
    })
}

export async function get_usuario(id_usuario){
    return get(`usuario/${id_usuario}`)
}

export async function cadastrar_categoria(id_documento, nome){
    return post(`categoria`, {
        id_documento,
        nome
    })
}

export async function deletar_categoria(id_categoria){
    return delete_(`categoria/${id_categoria}`)
}