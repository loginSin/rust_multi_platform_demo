package com.qxb.mynativelib.internal.guard.annotation;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

/**
 * 初始化保护注解，主要用于检测 CoreClientImpl 类是否初始化
 * 对比看 CoreClientImpl.java 和 CoreClientImpl.class
 */
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
public @interface InitGuard {
}
