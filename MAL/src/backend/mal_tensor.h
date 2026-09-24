// mal_tensor.h - Lightweight C tensor library for MAL Backend
#ifndef MAL_TENSOR_H
#define MAL_TENSOR_H
#include <stdint.h>
#include <stdbool.h>
typedef enum { DT_F32, DT_F64, DT_I32, DT_I64, DT_BOOL } Dtype;
typedef struct { int ndim; int* dims; } Shape;
typedef struct {
    void* data;
    Shape shape;
    Dtype dtype;
    bool requires_grad;
    void* grad;
} Tensor;
// Creation
Tensor tensor_zeros(int ndim, int* dims, Dtype dtype);
Tensor tensor_ones(int ndim, int* dims, Dtype dtype);
// Element-wise
Tensor tensor_add(Tensor a, Tensor b);
Tensor tensor_sub(Tensor a, Tensor b);
Tensor tensor_mul(Tensor a, Tensor b);
Tensor tensor_relu(Tensor a);
Tensor tensor_sigmoid(Tensor a);
// Linear Algebra
Tensor tensor_matmul(Tensor a, Tensor b);
Tensor tensor_transpose(Tensor a);
// Reshape
Tensor tensor_reshape(Tensor a, int new_ndim, int* new_dims);
// Autodiff
void tensor_backward(Tensor loss);
Tensor tensor_grad(Tensor loss, const char* var_name);
void tensor_zero_grad(Tensor* t);
// Memory
void tensor_free(Tensor t);
#endif
