package com.example.dao;

import java.sql.CallableStatement;
import java.sql.Connection;
import java.sql.Types;
import java.util.Map;

public class CallableExecutor {

    /**
     * JDBC {call} escape syntax with IN params.
     */
    public void callProcedure(Connection conn, int id, String name) throws Exception {
        String sql = "{call pkg_xxx.proc_yyy(?, ?)}";
        try (CallableStatement cs = conn.prepareCall(sql)) {
            cs.setInt(1, id);
            cs.setString(2, name);
            cs.execute();
        }
    }

    /**
     * JDBC {? = call ...} escape syntax with return value.
     */
    public Map<String, Object> callFunction(Connection conn, int id) throws Exception {
        String sql = "{? = call pkg_xxx.get_user(?)}";
        try (CallableStatement cs = conn.prepareCall(sql)) {
            cs.registerOutParameter(1, Types.OTHER);
            cs.setInt(2, id);
            cs.execute();
            return (Map<String, Object>) cs.getObject(1);
        }
    }

    /**
     * Standard CALL without JDBC escape (some drivers accept raw CALL).
     */
    public void callRaw(Connection conn) throws Exception {
        String sql = "CALL pkg_xxx.cleanup()";
        try (CallableStatement cs = conn.prepareCall(sql)) {
            cs.execute();
        }
    }

    /**
     * MyBatis-like dynamic SQL with {call} injected via string builder.
     */
    public void callDynamic(Connection conn, String procName, String param) throws Exception {
        String sql = "{call " + procName + "(" + param + ")}";
        try (CallableStatement cs = conn.prepareCall(sql)) {
            cs.execute();
        }
    }
}
