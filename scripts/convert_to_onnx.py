#!/usr/bin/env python3
"""
ONNX Conversion Script for Eidos Models

Converts trained PyTorch/Transformers models to ONNX format
for deployment with tract runtime.
"""

import argparse
import sys
from pathlib import Path

try:
    import torch
    from transformers import T5Tokenizer, T5ForConditionalGeneration
    import onnx
    from onnxsim import simplify
except ImportError as e:
    print(f"Error: Required library not installed: {e}")
    print("Install with: pip install torch transformers onnx onnx-simplifier")
    sys.exit(1)


def convert_to_onnx(model_path, output_path="model.onnx", simplify_model=True):
    """Convert model to ONNX format."""
    print("=" * 80)
    print("ONNX CONVERSION")
    print("=" * 80)
    print(f"Input model: {model_path}")
    print(f"Output file: {output_path}")
    print("=" * 80)
    print()

    # Load model and tokenizer
    print("Loading model and tokenizer...")
    try:
        model = T5ForConditionalGeneration.from_pretrained(model_path)
        tokenizer = T5Tokenizer.from_pretrained(model_path)
    except Exception as e:
        print(f"Error loading model: {e}")
        print("\nMake sure the model path contains a valid Transformers model.")
        sys.exit(1)

    model.eval()

    # Create dummy input
    print("Creating dummy input...")
    dummy_text = "translate to command: list files"
    dummy_input = tokenizer(dummy_text, return_tensors="pt", padding=True)
    input_ids = dummy_input['input_ids']
    attention_mask = dummy_input['attention_mask']

    # Export to ONNX
    print("Exporting to ONNX format...")
    try:
        with torch.no_grad():
            torch.onnx.export(
                model,
                (input_ids, attention_mask),
                output_path,
                input_names=['input_ids', 'attention_mask'],
                output_names=['output'],
                dynamic_axes={
                    'input_ids': {0: 'batch_size', 1: 'sequence'},
                    'attention_mask': {0: 'batch_size', 1: 'sequence'},
                    'output': {0: 'batch_size', 1: 'sequence'}
                },
                opset_version=14,
                do_constant_folding=True,
                verbose=False
            )
        print(f"✓ Model exported to: {output_path}")
    except Exception as e:
        print(f"Error during export: {e}")
        sys.exit(1)

    # Get model size
    onnx_size = Path(output_path).stat().st_size / (1024 * 1024)  # MB
    print(f"  Model size: {onnx_size:.2f} MB")

    # Simplify model (optional)
    if simplify_model:
        print("\nSimplifying ONNX model...")
        try:
            onnx_model = onnx.load(output_path)
            model_simp, check = simplify(onnx_model)

            if check:
                simplified_path = str(output_path).replace('.onnx', '_simplified.onnx')
                onnx.save(model_simp, simplified_path)
                simplified_size = Path(simplified_path).stat().st_size / (1024 * 1024)
                print(f"✓ Simplified model saved to: {simplified_path}")
                print(f"  Simplified size: {simplified_size:.2f} MB")
                print(f"  Size reduction: {onnx_size - simplified_size:.2f} MB")

                # Optionally replace original with simplified
                print("\nReplace original with simplified version? (y/n): ", end="")
                response = input().strip().lower()
                if response == 'y':
                    Path(simplified_path).replace(output_path)
                    print(f"✓ Replaced {output_path} with simplified version")
            else:
                print("⚠ Simplification validation failed, keeping original")
        except Exception as e:
            print(f"⚠ Simplification failed: {e}")
            print("Keeping original model")

    # Save tokenizer
    tokenizer_dir = Path(output_path).parent / "tokenizer"
    print(f"\nSaving tokenizer to: {tokenizer_dir}")
    tokenizer.save_pretrained(tokenizer_dir)
    print(f"✓ Tokenizer saved")

    # Print usage instructions
    print("\n" + "=" * 80)
    print("CONVERSION COMPLETE!")
    print("=" * 80)
    print(f"\nTo use with Eidos:")
    print(f"  1. Copy {output_path} to your Eidos directory")
    print(f"  2. Copy {tokenizer_dir}/tokenizer.json to your Eidos directory")
    print(f"  3. Configure in eidos.toml:")
    print(f"     model_path = \"{Path(output_path).name}\"")
    print(f"     tokenizer_path = \"tokenizer.json\"")
    print(f"\nOr set environment variables:")
    print(f"  export EIDOS_MODEL_PATH={Path(output_path).absolute()}")
    print(f"  export EIDOS_TOKENIZER_PATH={tokenizer_dir.absolute()}/tokenizer.json")


def main():
    parser = argparse.ArgumentParser(
        description="Convert Transformers model to ONNX format for Eidos"
    )
    parser.add_argument(
        'model_path',
        help="Path to trained Transformers model directory"
    )
    parser.add_argument(
        '-o', '--output',
        default='model.onnx',
        help="Output ONNX file path (default: model.onnx)"
    )
    parser.add_argument(
        '--no-simplify',
        action='store_true',
        help="Skip ONNX model simplification"
    )

    args = parser.parse_args()

    if not Path(args.model_path).exists():
        print(f"Error: Model path not found: {args.model_path}")
        sys.exit(1)

    convert_to_onnx(
        args.model_path,
        args.output,
        simplify_model=not args.no_simplify
    )


if __name__ == '__main__':
    main()
