package com.qxb.mynativelib.internal.guard.processor;

import javassist.*;
import javassist.bytecode.AnnotationsAttribute;
import javassist.bytecode.MethodInfo;
import javassist.bytecode.ParameterAnnotationsAttribute;
import javassist.bytecode.annotation.Annotation;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Objects;

public class GuardProcessor {
    private static String outputPath = null;
    private static final String initGuardPath = "com.qxb.mynativelib.internal.guard.annotation.InitGuard";
    private static final String paramGuardPath = "com.qxb.mynativelib.internal.guard.annotation.ParamGuard";
    private static final String[] callbackNameArray =
            {
                    "com/qxb/imlib/mynativelib/ICallback",
                    "com/qxb/imlib/mynativelib/IData0Callback",
                    "com/qxb/imlib/mynativelib/IData1Callback",
                    "com/qxb/imlib/mynativelib/IData2Callback",
                    "com/qxb/imlib/mynativelib/IData3Callback",
                    "com/qxb/imlib/mynativelib/IData4Callback"
            };

    public static void main(String[] args) throws NotFoundException, CannotCompileException, IOException {
        System.out.println("GuardProcessor main");

        if (args.length < 2) {
            System.out.println("Usage: GuardProcessor <inputDir> <outputDir>");
            return;
        }
        String inputPath = args[0];
        outputPath = args[1];

        File inputDir = new File(inputPath);
        File outputDir = new File(outputPath);

        processClasses(inputDir, outputDir);
    }

    private static void processClasses(File inputDir, File outputDir) throws NotFoundException, CannotCompileException, IOException {
        ClassPool pool = ClassPool.getDefault();
        pool.appendClassPath(inputDir.getAbsolutePath());
        System.out.println("GuardProcessor processClasses");

        // 递归处理目录下的所有文件
        processDirectory(pool, inputDir, outputDir, inputDir);
    }

    private static void processDirectory(ClassPool pool, File dir, File outputDir, File baseDir) throws NotFoundException, CannotCompileException, IOException {
        for (File file : Objects.requireNonNull(dir.listFiles())) {
            if (file.isDirectory()) {
                // 递归处理子目录
                processDirectory(pool, file, outputDir, baseDir);
            } else if (file.isFile() && file.getName().endsWith(".class")) {
                System.out.println("==========class start==========");

                // 处理 .class 文件
                String classFilePath = file.getAbsolutePath();
                String className = getClassName(baseDir, classFilePath);

                System.out.println("Processing class: " + className);

                CtClass cc = pool.get(className);

                processCtClass(cc, outputDir, baseDir, file);

                System.out.println("==========class end==========\n");

            }
        }
    }

    private static void processCtClass(CtClass cc, File outputDir, File baseDir, File file) throws CannotCompileException, IOException {
        // 处理方法及其注解
        for (CtMethod method : cc.getDeclaredMethods()) {
            System.out.println("\n----------method start----------");

            String methodName = method.getName();
            String signature = method.getSignature();
            System.out.println("method name is " + methodName + "(" + signature + ")");

            boolean hasCallback = hasCallbackInMethod(signature);

            // 获取 InitGuard 方法注解的代码段
            String initGuardCode = getInitGuardCode(method, hasCallback);

            // 获取 ParamGuard 参数注解的代码段
            ArrayList<String> paramCodeList = getParamGuardCode(method, hasCallback);

            // 插入代码
            insertCodeToMethod(method, initGuardCode, paramCodeList);

            System.out.println("----------method end----------\n");
        }

        writeToClass(cc, outputDir, baseDir, file);
    }

    private static void insertCodeToMethod(CtMethod method, String initGuardCode, ArrayList<String> paramCodeList) throws CannotCompileException {
        ArrayList<String> newCodeList = new ArrayList<>();
        newCodeList.add(initGuardCode);
        newCodeList.addAll(paramCodeList);

        System.out.println("start insert code");
        for (int i = newCodeList.size() - 1; i >= 0; i--) {
            String code = newCodeList.get(i);
            System.out.println("insertBefore code is: " + code);
            if (!code.isEmpty()) {
                method.insertBefore(code);
            }
        }

    }

    private static void writeToClass(CtClass cc, File outputDir, File baseDir, File file) throws CannotCompileException, IOException {
        // 计算相对路径并写入输出目录
        String relativePath = getRelativePath(baseDir, file);
        System.out.println("getRelativePath is: " + relativePath);
        File outputFile = new File(outputDir, relativePath);
        System.out.println("outputFile is: " + outputFile.getAbsolutePath());
        if (!outputFile.getParentFile().exists()) {
            boolean isMk = outputFile.getParentFile().mkdirs();
            System.out.println("outputFile.getParentFile() : " + isMk);
        }

        cc.writeFile(outputPath);
        System.out.println("Modified file written to: " + outputPath);
    }

    private static String getClassName(File baseDir, String classFilePath) {
        String classFilePathRelative = classFilePath.substring(baseDir.getAbsolutePath().length() + 1);
        classFilePathRelative = classFilePathRelative.replace(File.separatorChar, '.');
        return classFilePathRelative.substring(0, classFilePathRelative.length() - ".class".length());
    }


    private static String getRelativePath(File baseDir, File file) {
        // 获取相对于 baseDir 的相对路径
        // 保证相对路径只包含一次 baseDir
        return file.getAbsolutePath().substring(baseDir.getAbsolutePath().length() + 1);
    }

    // 获取 InitParam 代码段
    private static String getInitGuardCode(CtMethod method, boolean hasCallback) throws CannotCompileException {
        String initGuardCode = "";
        // 检查方法是否有 @InitGuard 注解
        MethodInfo methodInfo = method.getMethodInfo();
        AnnotationsAttribute attr = (AnnotationsAttribute) methodInfo.getAttribute(AnnotationsAttribute.visibleTag);
        if (attr != null && attr.getAnnotation(initGuardPath) != null) {
            System.out.println("Adding InitGuard for method: " + method.getName());
            initGuardCode = genInitGuardCode(hasCallback);
        }
        return initGuardCode;
    }

    // 生成新方法以生成初始化保护代码
    private static String genInitGuardCode(boolean hasCallback) {
        String code = "";
        String isInitString = "io.rong.imlib.internal.CoreClientImpl.getInstance().isInit()";
        if (hasCallback) {
            String notInitCodeString = "io.rong.imlib.enums.ErrorCode.NotInit";
            code = "{ if (! " + isInitString + ") { if (callback != null) { callback.onError(" + notInitCodeString + "); } return; } }";
        } else {
            code = "{ if (!" + isInitString + ") return; }";
        }
        System.out.println("getInitGuardCode : " + code);
        return code;
    }

    // 获取一个方法所有的 ParamGuard 代码
    private static ArrayList<String> getParamGuardCode(CtMethod method, boolean hasCallback) throws CannotCompileException {
        ArrayList<String> codeList = new ArrayList<>();
        MethodInfo methodInfo = method.getMethodInfo();
        ParameterAnnotationsAttribute paramAttr = (ParameterAnnotationsAttribute) methodInfo.getAttribute(ParameterAnnotationsAttribute.visibleTag);

        if (paramAttr != null) {
            Annotation[][] annotations = paramAttr.getAnnotations();
            for (int i = 0; i < annotations.length; i++) {
                for (Annotation annotation : annotations[i]) {
                    if (annotation.getTypeName().equals(paramGuardPath)) {
                        String errorCodeString = getParamGuardMemberValueString(annotation, "value");
                        String logTagString = getParamGuardMemberValueString(annotation, "logTag");

                        System.out.println("Adding ParamGuard for method: " + method.getName() + " parameter index: " + i + " value : " + errorCodeString);
                        String newCode = genParamGuardCode(method, i, hasCallback, errorCodeString, logTagString);
                        codeList.add(newCode);
                    }
                }
            }
        }

        return codeList;
    }

    private static String getParamGuardMemberValueString(Annotation annotation, String memberValueName) {
        Object valueObj = annotation.getMemberValue(memberValueName);
        if (valueObj != null) {
            return valueObj.toString();
        }
        return "";
    }

    // 生成 ParamGuard 代码
    private static String genParamGuardCode(CtMethod method, int paramIndex, boolean hasCallback, String errorCodeString, String logTagString) {
        String logString = "System.out.println(" + logTagString + " + \":\" + " + errorCodeString + ");";
        String code = "{ if ($" + (paramIndex + 1) + " == null) " + logString + " return; }";
        if (hasCallback) {
            code = "{ if ($" + (paramIndex + 1) + " == null) { " + logString + "if (callback != null) { callback.onError( " + errorCodeString + "); return;}  }}";
        }
        System.out.println("getParamCheckCode : " + code);
        return code;
    }

    private static boolean hasCallbackInMethod(String signature) {
        for (String callbackName : callbackNameArray) {
            if (signature.contains(callbackName)) {
                return true;
            }
        }
        return false;
    }
}