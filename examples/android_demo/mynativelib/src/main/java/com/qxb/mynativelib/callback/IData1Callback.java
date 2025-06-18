package com.qxb.mynativelib.callback;

/**
 * @author qi
 */
public interface IData1Callback<T> extends ICallback {
    /**
     * 成功回调，返回一个数据
     *
     * @param data 数据
     */
    void onSuccess(T data);
}
