use reqwest::{
    self,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ResponseDebida {
    #[serde(rename = "Entidad")]
    entidad: String,
    #[serde(rename = "Nombres")]
    nombres: String,
    #[serde(rename = "NroDocumento")]
    dni: String,
    #[serde(rename = "Oficina")]
    oficina: String,
    #[serde(rename = "PrimerApellido")]
    a_paterno: String,
    #[serde(rename = "SegundoApellido")]
    a_materno: String,
    #[serde(rename = "TipoDocumento")]
    tipo_doc: String,
    #[serde(rename = "UsuarioAudi")]
    usuario: String,
}

#[derive(Deserialize, Serialize)]
struct ResponseDni {
    #[serde(rename = "NroDocumento")]
    numero_documento: String,
    #[serde(rename = "UsuarioAudi")]
    usuario: String,
    #[serde(rename = "Oficina")]
    oficina: String,
    #[serde(rename = "Entidad")]
    entidad: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    #[serde(rename = "IsSuccess")]
    is_success: bool,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Description")]
    descrip: Option<String>,
    #[serde(rename = "DescriptionGeneral")]
    descrip_gen: Option<String>,
    #[serde(rename = "Exception")]
    excep: Option<String>,
    #[serde(rename = "Result")]
    result: Vec<ResultItem>,
    #[serde(rename = "Codigo")]
    cod: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResultItem {
    #[serde(rename = "FechaConsulta")]
    fecha_consulta: String,
    #[serde(rename = "TipoDocumento")]
    tipo_doc: String,
    #[serde(rename = "NroDocumento")]
    numero_documento: String,
    #[serde(rename = "ApellidoPaterno")]
    a_paterno: String,
    #[serde(rename = "ApellidoMaterno")]
    a_materno: String,
    #[serde(rename = "Nombres")]
    nombre: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VerificationResponse {
    #[serde(rename = "IsSuccess")]
    is_success: bool,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "DescriptionGeneral")]
    description_general: Option<String>,
    #[serde(rename = "Exception")]
    exception: Option<String>,
    #[serde(rename = "Result")]
    result: VerificationResult,
    #[serde(rename = "Codigo")]
    codigo: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VerificationResult {
    #[serde(rename = "Penales")]
    penales: PenalVerification,
    #[serde(rename = "Policiales")]
    policiales: PolicialVerification,
    #[serde(rename = "Judiciales")]
    judiciales: JudicialVerification,
    #[serde(rename = "Osce")]
    osce: Option<String>,
    #[serde(rename = "Servir")]
    servir: ServirVerification,
    #[serde(rename = "Minjus")]
    minjus: MinjusVerification,
}

#[derive(Debug, Deserialize, Serialize)]
struct PenalVerification {
    #[serde(rename = "IsSuccess")]
    is_success: Option<bool>,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "DescriptionGeneral")]
    description_general: Option<String>,
    #[serde(rename = "Exception")]
    exception: Option<String>,
    #[serde(rename = "Result")]
    result: Option<String>,
    #[serde(rename = "Codigo")]
    codigo: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct PolicialVerification {
    #[serde(rename = "IsSuccess")]
    is_success: Option<bool>,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "DescriptionGeneral")]
    description_general: Option<String>,
    #[serde(rename = "Exception")]
    exception: Option<String>,
    #[serde(rename = "Result")]
    result: Option<String>,
    #[serde(rename = "Codigo")]
    codigo: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct JudicialVerification {
    #[serde(rename = "IsSuccess")]
    is_success: Option<bool>,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "DescriptionGeneral")]
    description_general: Option<String>,
    #[serde(rename = "Exception")]
    exception: Option<String>,
    #[serde(rename = "Result")]
    result: Option<String>,
    #[serde(rename = "Codigo")]
    codigo: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct ServirVerification {
    #[serde(rename = "IsSuccess")]
    is_success: Option<bool>,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "DescriptionGeneral")]
    description_general: Option<String>,
    #[serde(rename = "Exception")]
    exception: Option<String>,
    #[serde(rename = "Result")]
    result: Option<String>,
    #[serde(rename = "Codigo")]
    codigo: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct MinjusVerification {
    #[serde(rename = "IsSuccess")]
    is_success: Option<bool>,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "DescriptionGeneral")]
    description_general: Option<String>,
    #[serde(rename = "Exception")]
    exception: Option<String>,
    #[serde(rename = "Result")]
    result: Option<String>,
    #[serde(rename = "Codigo")]
    codigo: Option<String>,
}

#[derive(Debug)]
enum MyError {
    Unauthorized,
}

// Implementa std::fmt::Display y std::error::Error para tu tipo de error
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unauthorized")
    }
}

impl std::error::Error for MyError {}

#[tokio::main]
async fn main() {
    let sa: Result<ApiResponse, Box<dyn std::error::Error>> =
        consultar_dni("xx dni a consultar xx").await;

    match sa {
        Ok(sa) => {
            let _ = consultar_debida(
                &sa.result[0].numero_documento,
                &sa.result[0].a_paterno,
                &sa.result[0].a_materno,
                &sa.result[0].nombre,
            )
            .await;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

async fn consultar_debida(
    dni: &str,
    aparterno: &str,
    amterno: &str,
    nombre: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let dni = ResponseDebida {
        entidad: "xxx".to_string(),
        oficina: "Oficina de Recursos Humanos".to_string(),
        usuario: "xxx".to_string(), // dni del usuario
        tipo_doc: "DNI".to_string(),
        a_materno: amterno.to_string(),
        a_paterno: aparterno.to_string(),
        dni: dni.to_string(),
        nombres: nombre.to_string(),
    };

    let mut heardes = HeaderMap::new();

    heardes.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    heardes.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    heardes.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Bearer //token va aqui//"),
    );

    let response = client
        .post(
            "
        https://debidadiligencia.servicios.gob.pe/servicioDebida/api/Debida/PostConsultarInforme/",
        )
        .headers(heardes)
        .body(serde_json::to_string(&dni)?)
        .send()
        .await;

    match response {
        Ok(e) => {
            if e.status() == 401 {
                return Err(Box::new(MyError::Unauthorized));
            }

            let strres = &e.text().await.unwrap();

            let res: VerificationResponse = serde_json::from_str(&strres).expect("algo");

            println!("{:?}", res.result.judiciales);
            println!("{:?}", res.result.penales);
            println!("{:?}", res.result.policiales);
            println!("{:?}", res.result.servir);

            Ok(())
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(Box::new(e));
        }
    }
}

async fn consultar_dni(dni: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let dni = ResponseDni {
        entidad: "xx".to_string(),
        numero_documento: dni.to_string(),
        oficina: "Oficina de Recursos Humanos".to_string(),
        usuario: "xxx".to_string(),
    };

    let mut heardes = HeaderMap::new();

    heardes.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    heardes.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    heardes.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Bearer //token va aqui//"),
    );

    let response = client.post(
        "https://debidadiligencia.servicios.gob.pe/servicioDebida/api/Debida/PostConsultPerson/",
    )
    .headers(heardes)
    .body(serde_json::to_string(&dni)?)
    .send().await;

    match response {
        Ok(e) => {
            if e.status() == 401 {
                return Err(Box::new(MyError::Unauthorized));
            }

            let res: ApiResponse = serde_json::from_str(&e.text().await.unwrap()).unwrap();

            Ok(res)
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(Box::new(e));
        }
    }
    // println!("{:?}",response.text().await?);
    // Ok(())
}
