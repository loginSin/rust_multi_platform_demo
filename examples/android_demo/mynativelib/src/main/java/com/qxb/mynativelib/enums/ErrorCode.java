package com.qxb.mynativelib.enums;

public enum ErrorCode {
    Unknown(-100),
    Success(0),
    NotInit(1),

    InvalidToken(100),

    InvalidContent(200),

    InvalidRoomId(300),

    InvalidLocalPath(400);

    private int code;

    ErrorCode(int code) {
        this.code = code;
    }
}
