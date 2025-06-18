package com.qxb.mynativelib.callback;

/**
 * @author qi
 */
public interface IData2Callback<T, V> extends ICallback {
    /**
     * 成功回调，两个数据
     *
     * @param data1 数据1
     * @param data2 数据2
     */
    void onSuccess(T data1, V data2);
}
