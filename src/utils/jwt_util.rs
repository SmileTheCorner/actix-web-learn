use chrono::Utc;
use jsonwebtoken::{errors::Error as JwtError,Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize,Deserialize};

const JWT_SECRET: &[u8] = b"Jwt_Secret";


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub id: String,
    pub name: String,
}

impl Claims {
    fn new(exp: usize,iat: usize,sub: String,id: String,name: String,)->Self{
        Self {exp,iat,sub,id,name}
    }
}

//创建jwt
pub fn create_token(id:String,name:String)->String{
    //过期时间
    let expiration =  Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("no valid timestamp")
        .timestamp();
    let header = Header::new(Algorithm::HS256);
    let iat = Utc::now().timestamp();
    let claims =  Claims::new(expiration as usize,iat as usize,"generate-token".to_string(),id,name);
    //生成token
    let token = jsonwebtoken::encode(&header,&claims,&EncodingKey::from_secret(JWT_SECRET)).map(|s| format!("Bearer {}", s)).unwrap();
    token
}

pub fn validate_token(token:String)->Result<Claims,JwtError> {
    let validation = Validation::new(Algorithm::HS256);
    let key = DecodingKey::from_secret(JWT_SECRET);
    let data = jsonwebtoken::decode::<Claims>(token.as_str(),&key,&validation)?;
    Ok(data.claims)
}
