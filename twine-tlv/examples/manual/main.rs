//! Example of manually implementing types that only represent a single TLV type.

use twine_tlv::prelude::*;

mod example_data;
mod more_example_data;
mod multi_example_data;
mod single_tuple;
mod single_tuple_multi_variant;
mod variable_struct;

use example_data::ExampleData;
use more_example_data::MoreExampleData;
use multi_example_data::{MultiExampleData, Variant0MultiExampleData, Variant1MultiExampleData};
use single_tuple::SingleTupleExampleData;
use single_tuple_multi_variant::{
    Variant0SingleTupleMultiVariant, Variant1SingleTupleMultiVariant,
};

use variable_struct::VariableStruct;

use crate::single_tuple_multi_variant::SingleTupleMultiVariant;

fn main() {
    let data0 = ExampleData::new(0x2A, 0xDEAD);
    let data1 = MoreExampleData::new(0xDAFF_0D11);

    let data3 = MultiExampleData::new(0x2A, 0xC0DE);
    let variant_data3: Variant0MultiExampleData = data3.into();

    let data4 = MultiExampleData::new(0x27, 0xFFFF);
    let variant_data4: Variant1MultiExampleData = data4.into();

    let data5 = SingleTupleExampleData::new(0xDEAD_BEEF);

    let data6 = SingleTupleMultiVariant::new(0xFEED_FACE);
    let variant_data6_0: Variant0SingleTupleMultiVariant = data6.into();
    let variant_data6_1: Variant1SingleTupleMultiVariant = data6.into();

    let data7 = VariableStruct {
        inner0: vec![0x01, 0x02, 0x03],
        inner1: vec![0xDEAD, 0xBEEF],
    };

    let mut collection = TlvCollection::<64>::default();

    collection.push(data0).unwrap();
    collection.push(data1).unwrap();
    collection.push(variant_data3).unwrap();
    collection.push(variant_data4).unwrap();
    collection.push(data5).unwrap();
    collection.push(variant_data6_0).unwrap();
    collection.push(variant_data6_1).unwrap();
    collection.push(data7).unwrap();
    println!("Push data:\t\t{:02X?}", collection);

    let data2 = ExampleData::new(0x27, 0xFFFF);

    collection.replace_or_push(data2).unwrap();
    println!("Replace ExampleData:\t{:02X?}", collection);

    collection.remove::<ExampleData>();
    println!("Remove ExampleData:\t{:02X?}", collection);
}
