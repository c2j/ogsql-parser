
CREATE OR REPLACE PACKAGE BIGFUND.PKG_BM_2 is
 -- Public function and procedure declarations
 PROCEDURE prc_check_balance(p_in_checkBalance IN VARCHAR2,
                             p_in_accno        IN VARCHAR2,
                             p_out_code        OUT VARCHAR2,
                             p_out_msg         OUT VARCHAR2);

 PROCEDURE prc_check_balance_2(p_in_checkBalance IN VARCHAR2,
                               p_in_accno        IN VARCHAR2,
                               p_out_code        OUT VARCHAR2,
                               p_out_msg         OUT VARCHAR2);
end pkg_batchpay_management_2;
/
CREATE OR REPLACE PACKAGE BODY BIGFUND.PKG_BM_2 is
 PROCEDURE prc_check_balance(p_in_checkBalance IN VARCHAR2,
                             p_in_accno        IN VARCHAR2,
                             p_out_code        OUT VARCHAR2,
                             p_out_msg         OUT VARCHAR2) IS
   v_balance   NUMBER;
   v_in_checkBalance NUMBER;
 BEGIN
   p_out_code := 0;
   BEGIN
     SELECT to_number(p_in_checkBalance) INTO v_in_checkBalance FROM sys_dummy;
    EXCEPTION
     WHEN OTHERS THEN
       v_in_checkBalance := '';
   END;
   BEGIN
       UPDATE dat_dsr_submit_result t
          SET t.donef = '1'
        WHERE t.data_key = p_in_accno
          AND t.donef = '0'
          AND t.tm = (SELECT MAX(s.tm)
                     FROM dat_dsr_submit_result s
                    WHERE s.data_key = p_in_accno)--ȡ���µ�����
          AND rownum = 1
       RETURNING to_number(nvl(t.fields_value,0)) / 100.00 INTO v_balance;
       EXCEPTION
       -- ������Ҫ����p0002(δ�鵽����)�쳣���ֶ���into���ֶθ���ֵ������gauss��û��ƥ�䵽��������������»ᱨ����Ӱ�����sqlִ��
           WHEN  sqlstate 'P0002'  THEN
             v_balance = null;
   END;
   IF v_balance IS NULL THEN
     p_out_code := 1;
     p_out_msg := 'У�����ʧ��,��ȡ�������ʧ��';
   END IF;
   IF v_balance <  v_in_checkBalance THEN
     p_out_code := 1;
     p_out_msg := 'У�����ʧ��,�������Ϊ'||v_balance;
   END IF;
 EXCEPTION
   WHEN OTHERS THEN
     p_out_code := 1;
     p_out_msg := 'У�����ʧ��'||SQLERRM;
 END;

 PROCEDURE prc_check_balance_2(p_in_checkBalance IN VARCHAR2,
                               p_in_accno        IN VARCHAR2,
                               p_out_code        OUT VARCHAR2,
                               p_out_msg         OUT VARCHAR2) IS
   v_balance_str VARCHAR2(100);
   v_balance     NUMBER;
   v_in_checkBalance NUMBER;
 BEGIN
   p_out_code := 0;
   p_out_msg  := '��͸֧';
   BEGIN
     SELECT to_number(p_in_checkbalance) INTO v_in_checkbalance FROM sys_dummy;
   EXCEPTION
     WHEN OTHERS THEN
       v_in_checkbalance := '';
   END;
   BEGIN
       UPDATE dat_dsr_submit_result t
          SET t.donef = '1'
        WHERE t.data_key = p_in_accno
          AND t.donef = '0'
          AND rownum = 1
       RETURNING t.fields_value INTO v_balance_str;
        EXCEPTION
        -- ������Ҫ����p0002(δ�鵽����)�쳣���ֶ���into���ֶθ���ֵ������gauss��û��ƥ�䵽��������������»ᱨ����Ӱ�����sqlִ��
           WHEN sqlstate 'P0002'  THEN
             v_balance_str = '';
   END;
   IF v_balance_str IS NULL THEN
     p_out_code := 2;
     p_out_msg  := '��ȡ�������ʧ��';
     RETURN;
   END IF;

   v_balance := to_number(nvl(v_balance_str, 0)) / 100.00;

   IF v_balance < v_in_checkbalance THEN
     p_out_code := 1;
     p_out_msg  := '͸֧';
   END IF;
 EXCEPTION
   WHEN OTHERS THEN
     p_out_code := 3;
     p_out_msg  := 'У�����ʧ��' || SQLERRM;
 END;
end pkg_batchpay_management_2;
