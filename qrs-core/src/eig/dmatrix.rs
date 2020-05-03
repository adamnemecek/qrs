use crate::ComplexScalar;
use lapacke::Layout;
use lapack_traits::LapackScalar;
use nalgebra::{DMatrix, DVector, DMatrixSlice};
use num_traits::{Zero, One};
use crate::eig::{QEiger, EigQRep, EigRange, EigJob};
use crate::reps::matrix::{DenseQRep, Op};
use crate::eig::dense::{EigResolver, EigRangeData};
use crate::quantum::{QRep, QObj};

pub trait EigScalar: ComplexScalar + LapackScalar { }
impl<N> EigScalar for N where N: ComplexScalar + LapackScalar{ }

impl<N: ComplexScalar+LapackScalar> QEiger<N, DenseQRep<N>>
for EigResolver<N>{
    fn make_eiger(shape: <Op<N> as QObj<N>>::Dims, job: EigJob, range: EigRange<<N as ComplexScalar>::R>) -> Self {
        assert_eq!(shape.0, shape.1);

        EigResolver::new(shape.0 as u32, job, EigRangeData::from(range), Layout::ColumnMajor,
                         true)
    }

    fn eigh(&mut self, op: &Op<N>) -> (Vec<N::R>, Op<N>) {
        let mat = self.borrow_matrix();
        let sl: &mut [N]  = mat.as_slice_mut().unwrap();
        sl.copy_from_slice(op.as_slice());
        self.eig();
        let (vals, vecs) = (self.vals().clone().into_raw_vec(), self.vecs().clone());
        let sh = vecs.shape();
        let vecs = DMatrix::from_vec(sh[0], sh[1], vecs.into_raw_vec());

        (vals, vecs)
    }

}

impl<N: ComplexScalar+LapackScalar> EigQRep<N> for DenseQRep<N>{
    fn eig(op: &Op<N>) -> (Vec<N::R>, Self::OpRep) {
        let mut eiger : EigResolver<N> = QEiger::<N, DenseQRep<N>>::make_eiger(op.qdim(), EigJob::ValsVecs, EigRange::All);
        let mat = eiger.borrow_matrix();
        let sl: &mut [N]  = mat.as_slice_mut().unwrap();
        sl.copy_from_slice(op.as_slice());
        eiger.eig();

        let (vals, vecs) = (eiger.vals().clone().into_raw_vec(), eiger.vecs().clone());
        let sh = vecs.shape();
        let vecs = DMatrix::from_vec(sh[0], sh[1], vecs.into_raw_vec());

        (vals, vecs)
    }
}