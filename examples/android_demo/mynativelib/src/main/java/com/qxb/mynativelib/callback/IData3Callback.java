package com.qxb.mynativelib.callback;

/**
 * @author qi
 * @date 2024/8/8
 */
public interface IData3Callback<T, V, K> extends ICallback {
    /**
     * 成功回调，三个数据
     *
     * @param data1 数据1
     * @param data2 数据2
     * @param data3 数据3
     */
    void onSuccess(T data1, V data2, K data3);
}
