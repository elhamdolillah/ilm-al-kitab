// mal_tensor.c - Implementation of Lightweight C tensor library for MAL Backend
#include "mal_tensor.h"
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdio.h>
#include <cblas.h>
// ═══════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════
static int calc_numel(Shape shape) {
    int numel = 1;
    for (int i = 0; i < shape.ndim; i++) {
        numel *= shape.dims[i];
    }
    return numel;
}
static void* alloc_tensor_data(int numel, Dtype dtype) {
    size_t size = 0;
    switch (dtype) {
        case DT_F32: size = sizeof(float); break;
        case DT_F64: size = sizeof(double); break;
        case DT_I32: size = sizeof(int32_t); break;
        case DT_I64: size = sizeof(int64_t); break;
        case DT_BOOL: size = sizeof(bool); break;
    }
    return calloc(numel, size);
}
// ═══════════════════════════════════════════════════════════
// Creation
// ═══════════════════════════════════════════════════════════
Tensor tensor_zeros(int ndim, int* dims, Dtype dtype) {
    Tensor t;
    t.shape.ndim = ndim;
    t.shape.dims = (int*)malloc(ndim * sizeof(int));
    memcpy(t.shape.dims, dims, ndim * sizeof(int));
    t.dtype = dtype;
    t.requires_grad = false;
    t.grad = NULL;
    t.data = alloc_tensor_data(calc_numel(t.shape), dtype);
    return t;
}
Tensor tensor_ones(int ndim, int* dims, Dtype dtype) {
    Tensor t = tensor_zeros(ndim, dims, dtype);
    int numel = calc_numel(t.shape);
    if (dtype == DT_F32) {
        float* data = (float*)t.data;
        for (int i = 0; i < numel; i++) data[i] = 1.0f;
    } else if (dtype == DT_F64) {
        double* data = (double*)t.data;
        for (int i = 0; i < numel; i++) data[i] = 1.0;
    } else if (dtype == DT_I32) {
        int32_t* data = (int32_t*)t.data;
        for (int i = 0; i < numel; i++) data[i] = 1;
    } else if (dtype == DT_I64) {
        int64_t* data = (int64_t*)t.data;
        for (int i = 0; i < numel; i++) data[i] = 1;
    }
    return t;
}
// ═══════════════════════════════════════════════════════════
// Element-wise Operations (Simplified: assumes compatible shapes)
// ═══════════════════════════════════════════════════════════
Tensor tensor_add(Tensor a, Tensor b) {
    Tensor res = tensor_zeros(a.shape.ndim, a.shape.dims, a.dtype);
    int numel = calc_numel(a.shape);
    if (a.dtype == DT_F64 && b.dtype == DT_F64) {
        double* da = (double*)a.data;
        double* db = (double*)b.data;
        double* dr = (double*)res.data;
        for (int i = 0; i < numel; i++) dr[i] = da[i] + db[i];
    }
    // TODO: Add NumPy-style broadcasting logic here for production
    return res;
}
Tensor tensor_sub(Tensor a, Tensor b) {
    Tensor res = tensor_zeros(a.shape.ndim, a.shape.dims, a.dtype);
    int numel = calc_numel(a.shape);
    if (a.dtype == DT_F64 && b.dtype == DT_F64) {
        double* da = (double*)a.data;
        double* db = (double*)b.data;
        double* dr = (double*)res.data;
        for (int i = 0; i < numel; i++) dr[i] = da[i] - db[i];
    }
    return res;
}
Tensor tensor_mul(Tensor a, Tensor b) {
    Tensor res = tensor_zeros(a.shape.ndim, a.shape.dims, a.dtype);
    int numel = calc_numel(a.shape);
    if (a.dtype == DT_F64 && b.dtype == DT_F64) {
        double* da = (double*)a.data;
        double* db = (double*)b.data;
        double* dr = (double*)res.data;
        for (int i = 0; i < numel; i++) dr[i] = da[i] * db[i];
    }
    return res;
}
Tensor tensor_relu(Tensor a) {
    Tensor res = tensor_zeros(a.shape.ndim, a.shape.dims, a.dtype);
    int numel = calc_numel(a.shape);
    if (a.dtype == DT_F64) {
        double* da = (double*)a.data;
        double* dr = (double*)res.data;
        for (int i = 0; i < numel; i++) dr[i] = da[i] > 0 ? da[i] : 0.0;
    }
    return res;
}
Tensor tensor_sigmoid(Tensor a) {
    Tensor res = tensor_zeros(a.shape.ndim, a.shape.dims, a.dtype);
    int numel = calc_numel(a.shape);
    if (a.dtype == DT_F64) {
        double* da = (double*)a.data;
        double* dr = (double*)res.data;
        for (int i = 0; i < numel; i++) dr[i] = 1.0 / (1.0 + exp(-da[i]));
    }
    return res;
}
// ═══════════════════════════════════════════════════════════
// Linear Algebra
// ═══════════════════════════════════════════════════════════
Tensor tensor_matmul(Tensor a, Tensor b) {
    // Simplified 2D matrix multiplication for DT_F64
    // 🚀 PRODUCTION OPTIMIZATION: Replace this naive loop with cblas_dgemm 
    // from OpenBLAS for massive performance gains on large matrices.
    int m = a.shape.dims[0];
    int k = a.shape.dims[1];
    int n = b.shape.dims[1];
    int res_dims[] = {m, n};
    Tensor res = tensor_zeros(2, res_dims, DT_F64);
    if (a.dtype == DT_F64 && b.dtype == DT_F64) {
        double* da = (double*)a.data;
        double* db = (double*)b.data;
        double* dr = (double*)res.data;
        // 🚀 Production Optimization: OpenBLAS cblas_dgemm
        // C = alpha * A * B + beta * C
        // A is (m x k), B is (k x n), C is (m x n)
        cblas_dgemm(CblasRowMajor, CblasNoTrans, CblasNoTrans,
                    m, n, k, 1.0, da, k, db, n, 0.0, dr, n);
    }
    return res;
}
Tensor tensor_transpose(Tensor a) {
    // Simplified 2D transpose
    int r = a.shape.dims[0];
    int c = a.shape.dims[1];
    int res_dims[] = {c, r};
    Tensor res = tensor_zeros(2, res_dims, a.dtype);
    if (a.dtype == DT_F64) {
        double* da = (double*)a.data;
        double* dr = (double*)res.data;
        for (int i = 0; i < r; i++) {
            for (int j = 0; j < c; j++) {
                dr[j * r + i] = da[i * c + j];
            }
        }
    }
    return res;
}
// ═══════════════════════════════════════════════════════════
// Reshape & Autodiff
// ═══════════════════════════════════════════════════════════
Tensor tensor_reshape(Tensor a, int new_ndim, int* new_dims) {
    Tensor res;
    res.shape.ndim = new_ndim;
    res.shape.dims = (int*)malloc(new_ndim * sizeof(int));
    memcpy(res.shape.dims, new_dims, new_ndim * sizeof(int));
    res.dtype = a.dtype;
    res.requires_grad = a.requires_grad;
    res.grad = NULL; // Gradient tracking requires full graph implementation
    res.data = a.data; // Share data (creates a view, not a copy)
    return res;
}
void tensor_backward(Tensor loss) {
    (void)loss; // Suppress unused parameter warning
    // 🚀 PRODUCTION: This should traverse the computational graph 
    // in reverse topological order and call grad_fn for each node.
    printf("[Autodiff] Backward pass initiated for loss tensor.\n");
}
Tensor tensor_grad(Tensor loss, const char* var_name) {
    printf("[Autodiff] Fetching gradient for variable: %s\n", var_name);
    return loss; // Stub: returns a placeholder
}
void tensor_zero_grad(Tensor* t) {
    if (t->grad) {
        int numel = calc_numel(t->shape);
        size_t size = (t->dtype == DT_F64) ? sizeof(double) : sizeof(float);
        memset(t->grad, 0, numel * size);
    }
}
// ═══════════════════════════════════════════════════════════
// Memory Management
// ═══════════════════════════════════════════════════════════
void tensor_free(Tensor t) {
    free(t.shape.dims);
    free(t.data);
    if (t.grad) free(t.grad);
}
