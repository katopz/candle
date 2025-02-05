#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use candle::{Device, Result, Tensor};
use candle_onnx::onnx::{AttributeProto, GraphProto, ModelProto, NodeProto, ValueInfoProto};
use std::collections::HashMap;

const INPUT_X: &str = "x";
const INPUT_Y: &str = "y";
const OUTPUT_Z: &str = "z";

fn create_model_proto_with_graph(graph: Option<GraphProto>) -> ModelProto {
    ModelProto {
        metadata_props: vec![],
        training_info: vec![],
        functions: vec![],
        ir_version: 0,
        opset_import: vec![],
        producer_name: "".to_string(),
        producer_version: "".to_string(),
        domain: "".to_string(),
        model_version: 0,
        doc_string: "".to_string(),
        graph,
    }
}

#[test]
fn test_evaluation_fails_without_defined_graph() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(None);

    let inputs: HashMap<String, Tensor> = HashMap::new();

    match candle_onnx::simple_eval(&manual_graph, inputs) {
        Err(err) => assert_eq!(err.to_string(), "no graph defined in proto"),
        Ok(_) => panic!("Expected an error due to undefined graph"),
    }

    Ok(())
}

// "Add"
#[test]
fn test_add_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Add".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string(), INPUT_Y.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), Tensor::new(&[2.], &Device::Cpu)?);
    inputs.insert(INPUT_Y.to_string(), Tensor::new(&[2.], &Device::Cpu)?);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");
    let first = z
        .to_vec1::<f64>()?
        .to_vec()
        .get(0)
        .expect("Failed to get first element")
        .clone();
    assert_eq!(first, 4.0f64);

    Ok(())
}

// "Sub"
#[test]
fn test_sub_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Sub".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string(), INPUT_Y.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), Tensor::new(&[2.], &Device::Cpu)?);
    inputs.insert(INPUT_Y.to_string(), Tensor::new(&[2.], &Device::Cpu)?);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");
    let first = z
        .to_vec1::<f64>()?
        .to_vec()
        .get(0)
        .expect("Failed to get first element")
        .clone();
    assert_eq!(first, 0.0f64);

    Ok(())
}

// "Mul"
#[test]
fn test_mul_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Mul".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string(), INPUT_Y.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), Tensor::new(&[2.], &Device::Cpu)?);
    inputs.insert(INPUT_Y.to_string(), Tensor::new(&[2.], &Device::Cpu)?);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");
    let first = z
        .to_vec1::<f64>()?
        .to_vec()
        .get(0)
        .expect("Failed to get first element")
        .clone();
    assert_eq!(first, 4.0f64);

    Ok(())
}

// "Div"
#[test]
fn test_div_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Div".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string(), INPUT_Y.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), Tensor::new(&[2.], &Device::Cpu)?);
    inputs.insert(INPUT_Y.to_string(), Tensor::new(&[2.], &Device::Cpu)?);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");
    let first = z
        .to_vec1::<f64>()?
        .to_vec()
        .get(0)
        .expect("Failed to get first element")
        .clone();

    assert_eq!(first, 1.0f64);

    Ok(())
}

// "Equal"
#[test]
fn test_equal_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Equal".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string(), INPUT_Y.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), Tensor::new(&[2.], &Device::Cpu)?);
    inputs.insert(INPUT_Y.to_string(), Tensor::new(&[2.], &Device::Cpu)?);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");
    let first = z.to_dtype(candle::DType::U8)?.to_vec1::<u8>()?.to_vec()[0];
    assert_eq!(first, 1);

    Ok(())
}

// "Not"
#[test]
fn test_not_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Not".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), Tensor::new(&[0.], &Device::Cpu)?);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");
    let first = z.to_dtype(candle::DType::U8)?.to_vec1::<u8>()?.to_vec()[0];
    assert_eq!(first, 1);

    Ok(())
}

// "MatMul"
#[test]
fn test_matmul_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "MatMul".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string(), INPUT_Y.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(
        INPUT_X.to_string(),
        Tensor::from_vec(
            //
            vec![1.0f32, 2.0f32, 3.0f32, 4.0f32],
            &[2, 2],
            &Device::Cpu,
        )?,
    );
    inputs.insert(
        INPUT_Y.to_string(),
        Tensor::from_vec(
            //
            vec![5.0f32, 6.0f32, 7.0f32, 8.0f32],
            &[2, 2],
            &Device::Cpu,
        )?,
    );

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");
    let results = z.to_vec2::<f32>()?;
    assert_eq!(results, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);

    Ok(())
}

// "Reshape"
#[test]
fn test_reshape_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Reshape".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string(), INPUT_Y.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![
            ValueInfoProto {
                name: INPUT_X.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
            ValueInfoProto {
                name: INPUT_Y.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
        ],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let x = Tensor::from_vec(
        //
        vec![1.0f32, 2.0f32, 3.0f32, 4.0f32],
        &[2, 2],
        &Device::Cpu,
    )?;
    let y = Tensor::from_vec(
        //
        vec![4i64],
        &[1],
        &Device::Cpu,
    )?;

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), x);
    inputs.insert(INPUT_Y.to_string(), y);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");

    let results = z.to_vec1::<f32>()?;

    assert_eq!(results, vec![1.0, 2.0, 3.0, 4.0]);

    Ok(())
}

// "LogSoftmax"
#[test]
fn test_logsoftmax_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "LogSoftmax".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![
            ValueInfoProto {
                name: INPUT_X.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
            ValueInfoProto {
                name: INPUT_Y.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
        ],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let x = Tensor::from_vec(
        //
        vec![1.0f32, 2.0f32, 3.0f32, 4.0f32],
        &[2, 2],
        &Device::Cpu,
    )?;

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), x);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");

    let results = z.to_vec2::<f32>()?;

    assert_eq!(
        results,
        vec![vec![0.26894143, 0.7310586], vec![0.26894143, 0.7310586]]
    );

    Ok(())
}

// "Softmax"
#[test]
fn test_softmax_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Softmax".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![
            ValueInfoProto {
                name: INPUT_X.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
            ValueInfoProto {
                name: INPUT_Y.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
        ],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let x = Tensor::from_vec(
        //
        vec![1.0f32, 2.0f32, 3.0f32, 4.0f32],
        &[2, 2],
        &Device::Cpu,
    )?;

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), x);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");

    let results = z.to_vec2::<f32>()?;

    assert_eq!(
        results,
        vec![vec![0.26894143, 0.7310586], vec![0.26894143, 0.7310586]]
    );

    Ok(())
}

// "Transpose"
#[test]
fn test_transpose_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Transpose".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![
            ValueInfoProto {
                name: INPUT_X.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
            ValueInfoProto {
                name: INPUT_Y.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
        ],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let x = Tensor::from_vec(
        //
        vec![1.0f32, 2.0f32, 3.0f32, 4.0f32],
        &[2, 2],
        &Device::Cpu,
    )?;

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), x);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");

    let results = z.to_vec2::<f32>()?;

    assert_eq!(results, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);

    Ok(())
}

// "Dropout"
#[test]
fn test_dropout_operation() -> Result<()> {
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Dropout".to_string(),
            domain: "".to_string(),
            attribute: vec![],
            input: vec![INPUT_X.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![
            ValueInfoProto {
                name: INPUT_X.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
            ValueInfoProto {
                name: INPUT_Y.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
        ],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));
    let x = Tensor::from_vec(
        //
        vec![1.0f32, 2.0f32, 3.0f32, 4.0f32],
        &[2, 2],
        &Device::Cpu,
    )?;

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), x);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");

    let results = z.to_vec2::<f32>()?;

    assert_eq!(results, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    Ok(())
}

// "Flatten"
#[test]
fn test_flatten_operation() -> Result<()> {
    let mut att_axis = AttributeProto {
        name: "axis".to_string(),
        ref_attr_name: "axis".to_string(),
        i: 0,
        doc_string: "axis".to_string(),
        r#type: 2,
        f: 0.0,
        s: vec![],
        t: None,
        g: None,
        sparse_tensor: None,
        tp: None,
        floats: vec![],
        ints: vec![],
        strings: vec![],
        tensors: vec![],
        graphs: vec![],
        sparse_tensors: vec![],
        type_protos: vec![],
    };
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Flatten".to_string(),
            domain: "".to_string(),
            attribute: vec![att_axis.clone()],
            input: vec![INPUT_X.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![
            ValueInfoProto {
                name: INPUT_X.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
            ValueInfoProto {
                name: INPUT_Y.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
        ],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));
    let x = Tensor::from_vec(
        vec![
            1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 7.0f32, 8.0f32,
        ],
        &[2, 2, 2],
        &Device::Cpu,
    )?;

    let mut inputs: HashMap<String, Tensor> = HashMap::new();
    inputs.insert(INPUT_X.to_string(), x);

    let eval = candle_onnx::simple_eval(&manual_graph, inputs.clone())?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");

    let results = z.to_vec2::<f32>()?;

    assert_eq!(results, vec![vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]]);

    att_axis.i = 1;
    let manual_graph = create_model_proto_with_graph(Some(GraphProto {
        node: vec![NodeProto {
            op_type: "Flatten".to_string(),
            domain: "".to_string(),
            attribute: vec![att_axis.clone()],
            input: vec![INPUT_X.to_string()],
            output: vec![OUTPUT_Z.to_string()],
            name: "".to_string(),
            doc_string: "".to_string(),
        }],
        name: "".to_string(),
        initializer: vec![],
        input: vec![
            ValueInfoProto {
                name: INPUT_X.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
            ValueInfoProto {
                name: INPUT_Y.to_string(),
                doc_string: "".to_string(),
                r#type: None,
            },
        ],
        output: vec![ValueInfoProto {
            name: OUTPUT_Z.to_string(),
            doc_string: "".to_string(),
            r#type: None,
        }],
        value_info: vec![],
        doc_string: "".to_string(),
        sparse_initializer: vec![],
        quantization_annotation: vec![],
    }));

    let eval = candle_onnx::simple_eval(&manual_graph, inputs)?;
    assert_eq!(eval.len(), 1);

    let z = eval.get(OUTPUT_Z).expect("Output 'z' not found");

    let results = z.to_vec2::<f32>()?;

    assert_eq!(
        results,
        vec![vec![1.0, 2.0, 3.0, 4.0], vec![5.0, 6.0, 7.0, 8.0]]
    );

    Ok(())
}

// Below are ops that are implemented but not tested yet

// "MaxPool"
// #[test]

// "AveragePool"
// #[test]

// "BatchNormalization"
// #[test]

// "Squeeze"
// #[test]

// "ConstantOfShape"
// #[test]

// "Unsqueeze"
// #[test]

// "Clip"
// #[test]

// "Gather"
// #[test]

// "Shape"
// #[test]

// "Conv"
// #[test]

// "Concat"
// #[test]

// "Abs"
// #[test]

// "Cos"
// #[test]

// "Sin"
// #[test]

// "Neg"
// #[test]

// "Erf"
// #[test]

// "Tanh"
// #[test]

// "Sigmoid"
// #[test]

// "Gelu"
// #[test]

// "Relu"
// #[test]

// "Constant"
// #[test]

// "Cast"
// #[test]
