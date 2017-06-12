use rand;
use rand::Rng;
use rand::distributions::{Normal, IndependentSample};

pub enum Loss {
    Logistic,
    BPR,
}

struct Embeddings {
    rows: usize,
    latent_dim: usize,

    parameters: Vec<f32>,
    gradients: Vec<f32>,
    regularization: Vec<f32>,
}


impl Embeddings {
    fn new<T: Rng>(rows: usize, latent_dim: usize, rng: &mut T) -> Embeddings {

        let distribution = Normal::new(0.0, (latent_dim as f64).sqrt());

        Embeddings {
            rows: rows,
            latent_dim: latent_dim,
            parameters: (0..rows * latent_dim)
                .map(|x| distribution.ind_sample(rng) as f32)
                .collect::<Vec<_>>(),
            gradients: vec![1.0; rows * latent_dim],
            regularization: vec![1.0; rows * latent_dim],
        }
    }
}


pub struct Hyperparameters {
    loss: Loss,
    learning_rate: f32,
    l2: f32,
    num_users: usize,
    num_items: usize,
    latent_dim: usize,
}


pub struct ImplicitMatrixFactorization {
    hyperparameters: Hyperparameters,
    user_embeddings: Embeddings,
    item_embeddings: Embeddings,
}


impl Hyperparameters {
    pub fn new(num_users: usize, num_items: usize) -> Hyperparameters {
        Hyperparameters {
            loss: Loss::BPR,
            learning_rate: 0.05,
            l2: 0.0,
            num_users: num_users,
            num_items: num_items,
            latent_dim: 32,
        }
    }
    pub fn loss(&mut self, loss: Loss) -> &mut Self {
        self.loss = loss;
        self
    }
    pub fn latent_dim(&mut self, latent_dim: usize) -> &mut Self {
        self.latent_dim = latent_dim;
        self
    }
    pub fn build(self) -> ImplicitMatrixFactorization {

        let (num_users, num_items, latent_dim) = (self.num_users, self.num_items, self.latent_dim);
        let mut rng = rand::weak_rng();

        ImplicitMatrixFactorization {
            hyperparameters: self,
            user_embeddings: Embeddings::new(num_users, latent_dim, &mut rng),
            item_embeddings: Embeddings::new(num_items, latent_dim, &mut rng),
        }
    }
}
