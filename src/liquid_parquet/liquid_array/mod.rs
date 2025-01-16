mod primitive_array;
mod raw;
mod string_array;

use std::{any::Any, num::NonZero, sync::Arc};

use arrow::array::{ArrayRef, BooleanArray};
pub use primitive_array::{
    LiquidI16Array, LiquidI32Array, LiquidI64Array, LiquidI8Array, LiquidPrimitiveArray,
    LiquidPrimitiveMetadata, LiquidPrimitiveType, LiquidU16Array, LiquidU32Array, LiquidU64Array,
    LiquidU8Array,
};
use raw::bit_pack_array::BitPackedArray;
use raw::fsst_array::FsstArray;
pub use string_array::{EtcStringMetadata, LiquidStringArray};

/// A trait to access the underlying Liquid array.
pub trait AsLiquidArray {
    /// Get the underlying string array.
    fn as_string_array_opt(&self) -> Option<&LiquidStringArray>;

    /// Get the underlying string array.
    fn as_string(&self) -> &LiquidStringArray {
        self.as_string_array_opt().expect("liquid string array")
    }

    /// Get the underlying primitive array.
    fn as_primitive_array_opt<T: LiquidPrimitiveType>(&self) -> Option<&LiquidPrimitiveArray<T>>;

    /// Get the underlying primitive array.
    fn as_primitive<T: LiquidPrimitiveType>(&self) -> &LiquidPrimitiveArray<T> {
        self.as_primitive_array_opt()
            .expect("liquid primitive array")
    }
}

impl AsLiquidArray for dyn LiquidArray + '_ {
    fn as_string_array_opt(&self) -> Option<&LiquidStringArray> {
        self.as_any().downcast_ref()
    }

    fn as_primitive_array_opt<T: LiquidPrimitiveType>(&self) -> Option<&LiquidPrimitiveArray<T>> {
        self.as_any().downcast_ref()
    }
}

/// A Liquid array.
pub trait LiquidArray: std::fmt::Debug + Send + Sync {
    /// Get the underlying any type.
    fn as_any(&self) -> &dyn Any;

    /// Get the memory size of the ETC array.
    fn get_array_memory_size(&self) -> usize;

    /// Get the length of the ETC array.
    fn len(&self) -> usize;

    /// Check if the ETC array is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert the ETC array to an Arrow array.
    fn to_arrow_array(&self) -> ArrayRef;

    /// Convert the ETC array to an Arrow array.
    fn to_best_arrow_array(&self) -> ArrayRef {
        self.to_arrow_array()
    }

    /// Filter the ETC array with a boolean array.
    fn filter(&self, selection: &BooleanArray) -> LiquidArrayRef;
}

/// A reference to a Liquid array.
pub type LiquidArrayRef = Arc<dyn LiquidArray>;

pub(crate) fn get_bit_width(max_value: u64) -> NonZero<u8> {
    if max_value <= 1 {
        // todo: here we actually should return 0, as we should just use constant encoding.
        // but that's not implemented yet.
        NonZero::new(1).unwrap()
    } else {
        NonZero::new(64 - max_value.leading_zeros() as u8).unwrap()
    }
}
