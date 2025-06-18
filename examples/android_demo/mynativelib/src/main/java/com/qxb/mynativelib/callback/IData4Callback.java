package com.qxb.mynativelib.callback;

public interface IData4Callback<T, K, V, S> extends ICallback {
    /**
     * 成功回调，返回四个数据
     * @param data1 数据1
     * @param data2 数据2
     * @param data3 数据3
     * @param data4 数据4
     */
    void onSuccess(T data1, K data2, V data3, S data4);
}
