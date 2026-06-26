-- verify_gaussdb_functions.sql
-- Auto-generated from src/parser/function_registry.rs
-- Purpose: smoke-test every registered function on a real GaussDB instance.
--
-- Usage on GaussDB commercial (NOT openGauss-lite; many functions are commercial-only):
--   gsql -d <db> -f scripts/verify_gaussdb_functions.sql
--
-- Notes:
--   * Functions requiring a_format_dev_version are wrapped in SET/RESET pairs.
--   * Aggregate/window functions use a single-row source via (SELECT 1) t.
--   * Set-returning functions are called in FROM clause.
--   * Each statement is independent; failures do not abort the script.

SET a_format_version = '10c';
SET a_format_dev_version = 's2';
SET behavior_compat_options = 'proc_outparam_override';

-- ============================================================
-- Domain: Aggregate  (31 functions)
-- ============================================================
SELECT array_agg(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT avg(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT bit_and(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT bit_or(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT corr(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT count(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT covar_pop(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT covar_samp(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT every(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT max(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT median(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT min(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT mode(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT percentile_cont(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT percentile_disc(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_avgx(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_avgy(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_count(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_intercept(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_r2(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_slope(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_sxx(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_sxy(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT regr_syy(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT stddev(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT stddev_pop(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT stddev_samp(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT sum(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT var_pop(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT var_samp(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT variance(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate

-- ============================================================
-- Domain: Array  (29 functions)
-- ============================================================
SELECT array_append(NULL, NULL);
SELECT array_cat(NULL, NULL);
SELECT array_delete(NULL);
SELECT array_deleteidx(NULL, NULL);
SELECT array_dims(NULL);
SELECT array_except(NULL, NULL);
SELECT array_except_distinct(NULL, NULL);
SELECT array_exists(NULL, NULL);
SELECT array_extendnull(NULL, NULL);
SELECT array_intersect(NULL, NULL);
SELECT array_intersect_distinct(NULL, NULL);
SELECT array_length(NULL, NULL);
SELECT array_lower(NULL, NULL);
SELECT array_ndims(NULL);
SELECT array_next(NULL, NULL);
SELECT array_positions(NULL, NULL);
SELECT array_prepend(NULL, NULL);
SELECT array_prior(NULL, NULL);
SELECT array_sort(NULL);
SELECT array_to_string(NULL, NULL);
SELECT array_trim(NULL, NULL);
SELECT array_union(NULL, NULL);
SELECT array_union_distinct(NULL, NULL);
SELECT array_upper(NULL, NULL);
SELECT cardinality(NULL);
SELECT * FROM generate_subscripts(NULL, NULL);  -- set-returning
SELECT string_to_array(NULL, NULL);
SELECT * FROM unnest(NULL);  -- set-returning
SELECT * FROM unnest_table(NULL);  -- set-returning

-- ============================================================
-- Domain: Crypto  (11 functions)
-- ============================================================
SELECT aes_decrypt(NULL, NULL);
SELECT aes_encrypt(NULL, NULL);
SELECT digest(NULL, NULL);
SELECT gen_random_uuid();
SELECT gs_decrypt(NULL, NULL, NULL);
SELECT gs_decrypt_aes128(NULL, NULL);
SELECT gs_encrypt(NULL, NULL, NULL);
SELECT gs_encrypt_aes128(NULL, NULL);
SELECT md5(NULL, NULL);
SELECT sha1(NULL);
SELECT sha2(NULL, NULL);

-- ============================================================
-- Domain: DateTime  (66 functions)
-- ============================================================
SELECT adddate(NULL, NULL);
SELECT addtime(NULL, NULL);
SELECT age(NULL, NULL);
SELECT clock_timestamp();
SELECT convert_tz(NULL, NULL, NULL);
SELECT curdate();
SELECT current_date();
SELECT current_time(NULL);
SELECT current_timestamp(NULL);
SELECT curtime(NULL);
SELECT date_add(NULL, NULL);
SELECT date_format(NULL, NULL);
SELECT date_part(NULL, NULL);
SELECT date_sub(NULL, NULL);
SELECT date_trunc(NULL, NULL);
SELECT datediff(NULL, NULL);
SELECT dayname(NULL);
SELECT dayofmonth(NULL);
SELECT dayofweek(NULL);
SELECT dayofyear(NULL);
SELECT extract(NULL, NULL);
SELECT from_days(NULL);
SELECT from_unixtime(NULL, NULL);
SELECT isfinite(NULL);
SELECT justify_days(NULL);
SELECT justify_hours(NULL);
SELECT justify_interval(NULL);
SELECT localtime(NULL);
SELECT localtimestamp(NULL);
SELECT make_date(NULL, NULL, NULL);
SELECT make_time(NULL, NULL, NULL);
SELECT make_timestamp(NULL, NULL, NULL, NULL, NULL, NULL);
SELECT make_timestamptz(NULL, NULL, NULL, NULL, NULL, NULL);
SELECT makedate(NULL, NULL);
SELECT maketime(NULL, NULL, NULL);
SELECT monthname(NULL);
SELECT new_time(NULL, NULL, NULL);
SELECT now();
SELECT period_add(NULL, NULL);
SELECT period_diff(NULL, NULL);
SELECT sec_to_time(NULL);
SELECT statement_timestamp();
SELECT str_to_date(NULL, NULL);
SELECT subdate(NULL, NULL);
SELECT subtime(NULL, NULL);
SELECT sys_extract_utc(NULL);
SELECT time_format(NULL, NULL);
SELECT time_to_sec(NULL);
SELECT timediff(NULL, NULL);
SELECT timenow();
SELECT timeofday();
SELECT timestampadd(NULL, NULL, NULL);
SELECT timestampdiff(NULL, NULL, NULL);
SELECT to_days(NULL);
SELECT to_seconds(NULL);
SELECT to_timestamp(NULL, NULL);
SELECT to_timestamp_tz(NULL, NULL);
SELECT transaction_timestamp();
SELECT tz_offset(NULL);
SELECT unix_timestamp(NULL);
SELECT utc_date();
SELECT utc_time(NULL);
SELECT utc_timestamp(NULL);
SELECT weekday(NULL);
SELECT weekofyear(NULL);
SELECT yearweek(NULL, NULL);

-- ============================================================
-- Domain: DbeApplicationInfo  (5 functions)
-- ============================================================
CALL dbe_application_info.read_client_info(NULL);
CALL dbe_application_info.read_module(NULL, NULL);
CALL dbe_application_info.set_action(NULL);
CALL dbe_application_info.set_client_info(NULL);
CALL dbe_application_info.set_module(NULL, NULL);

-- ============================================================
-- Domain: DbeFile  (25 functions)
-- ============================================================
CALL dbe_file.close(NULL);
CALL dbe_file.close_all();
CALL dbe_file.copy(NULL, NULL, NULL);
CALL dbe_file.flush(NULL);
CALL dbe_file.fopen(NULL, NULL, NULL);
CALL dbe_file.fopen_nchar(NULL, NULL, NULL);
CALL dbe_file.format_write(NULL, NULL);
CALL dbe_file.format_write_nchar(NULL, NULL);
CALL dbe_file.get_attr(NULL, NULL, NULL, NULL, NULL);
CALL dbe_file.get_pos(NULL);
CALL dbe_file.get_raw(NULL, NULL);
CALL dbe_file.is_close(NULL);
CALL dbe_file.is_open(NULL);
CALL dbe_file.new_line(NULL, NULL);
CALL dbe_file.open(NULL, NULL);
CALL dbe_file.put_raw(NULL, NULL);
CALL dbe_file.read_line(NULL, NULL);
CALL dbe_file.read_line_nchar(NULL, NULL);
CALL dbe_file.remove(NULL, NULL);
CALL dbe_file.rename(NULL, NULL, NULL);
CALL dbe_file.seek(NULL, NULL);
CALL dbe_file.write(NULL, NULL);
CALL dbe_file.write_line(NULL, NULL);
CALL dbe_file.write_line_nchar(NULL, NULL);
CALL dbe_file.write_nchar(NULL, NULL);

-- ============================================================
-- Domain: DbeLob  (42 functions)
-- ============================================================
CALL dbe_lob.append(NULL, NULL);
CALL dbe_lob.bfileclose(NULL);
CALL dbe_lob.bfilename(NULL, NULL);
CALL dbe_lob.bfileopen(NULL, NULL);
CALL dbe_lob.close(NULL);
CALL dbe_lob.compare(NULL, NULL);
CALL dbe_lob.converttoblob(NULL, NULL);
CALL dbe_lob.converttoclob(NULL, NULL);
CALL dbe_lob.copy(NULL, NULL, NULL);
CALL dbe_lob.create_temporary(NULL, NULL);
CALL dbe_lob.erase(NULL, NULL);
CALL dbe_lob.fileclose(NULL);
CALL dbe_lob.fileopen(NULL, NULL);
CALL dbe_lob.freetemporary(NULL);
CALL dbe_lob.get_length(NULL);
CALL dbe_lob.getchunksize(NULL);
CALL dbe_lob.instr(NULL, NULL);
CALL dbe_lob.loadblobfrombfile(NULL, NULL, NULL, NULL, NULL);
CALL dbe_lob.loadblobfromfile(NULL, NULL, NULL, NULL, NULL);
CALL dbe_lob.loadclobfrombfile(NULL, NULL, NULL, NULL, NULL);
CALL dbe_lob.loadclobfromfile(NULL, NULL, NULL, NULL, NULL);
CALL dbe_lob.loadfrombfile(NULL, NULL, NULL);
CALL dbe_lob.loadfromfile(NULL, NULL, NULL, NULL, NULL);
CALL dbe_lob.lob_append(NULL, NULL);
CALL dbe_lob.lob_converttoblob(NULL, NULL, NULL, NULL, NULL);
CALL dbe_lob.lob_converttoclob(NULL, NULL, NULL, NULL, NULL);
CALL dbe_lob.lob_copy(NULL, NULL, NULL);
CALL dbe_lob.lob_erase(NULL, NULL);
CALL dbe_lob.lob_get_length(NULL);
CALL dbe_lob.lob_read(NULL, NULL, NULL);
CALL dbe_lob.lob_strip(NULL, NULL);
CALL dbe_lob.lob_substr(NULL, NULL);
CALL dbe_lob.lob_write(NULL, NULL, NULL, NULL);
CALL dbe_lob.lob_write_append(NULL, NULL);
CALL dbe_lob.match(NULL, NULL);
CALL dbe_lob.open(NULL, NULL);
CALL dbe_lob.read(NULL, NULL, NULL);
CALL dbe_lob.strip(NULL, NULL);
CALL dbe_lob.substr(NULL, NULL);
CALL dbe_lob.trim(NULL, NULL);
CALL dbe_lob.write(NULL, NULL, NULL);
CALL dbe_lob.write_append(NULL, NULL);

-- ============================================================
-- Domain: DbeMatch  (1 functions)
-- ============================================================
CALL dbe_match.edit_distance_similarity(NULL, NULL);

-- ============================================================
-- Domain: DbeOutput  (10 functions)
-- ============================================================
CALL dbe_output.disable();
CALL dbe_output.enable(NULL);
CALL dbe_output.get_line(NULL, NULL);
CALL dbe_output.get_lines(NULL, NULL);
CALL dbe_output.new_line();
CALL dbe_output.print(NULL);
CALL dbe_output.print_line(NULL);
CALL dbe_output.put(NULL);
CALL dbe_output.put_line(NULL);
CALL dbe_output.set_buffer_size(NULL);

-- ============================================================
-- Domain: DbeRandom  (1 functions)
-- GaussDB commercial only
-- ============================================================
CALL dbe_random.get_value(NULL, NULL);

-- ============================================================
-- Domain: DbeRaw  (26 functions)
-- ============================================================
CALL dbe_raw.bit_and(NULL, NULL);
CALL dbe_raw.bit_complement(NULL);
CALL dbe_raw.bit_or(NULL, NULL);
CALL dbe_raw.bit_xor(NULL, NULL);
CALL dbe_raw.cast_from_binary_double_to_raw(NULL, NULL);
CALL dbe_raw.cast_from_binary_float_to_raw(NULL, NULL);
CALL dbe_raw.cast_from_binary_integer_to_raw(NULL, NULL);
CALL dbe_raw.cast_from_number_to_raw(NULL);
CALL dbe_raw.cast_from_raw_to_binary_double(NULL, NULL);
CALL dbe_raw.cast_from_raw_to_binary_float(NULL, NULL);
CALL dbe_raw.cast_from_raw_to_binary_integer(NULL, NULL);
CALL dbe_raw.cast_from_raw_to_number(NULL);
CALL dbe_raw.cast_from_raw_to_nvarchar2(NULL);
CALL dbe_raw.cast_from_varchar2_to_raw(NULL);
CALL dbe_raw.cast_to_varchar2(NULL);
CALL dbe_raw.compare(NULL, NULL);
CALL dbe_raw.concat(NULL, NULL);
CALL dbe_raw.convert(NULL, NULL, NULL);
CALL dbe_raw.copies(NULL, NULL);
CALL dbe_raw.get_length(NULL);
CALL dbe_raw.overlay(NULL, NULL);
CALL dbe_raw.reverse(NULL);
CALL dbe_raw.substr(NULL, NULL);
CALL dbe_raw.translate(NULL, NULL, NULL);
CALL dbe_raw.transliterate(NULL, NULL);
CALL dbe_raw.xrange(NULL, NULL);

-- ============================================================
-- Domain: DbeScheduler  (31 functions)
-- ============================================================
CALL dbe_scheduler.create_credential(NULL, NULL, NULL);
CALL dbe_scheduler.create_job(NULL);
CALL dbe_scheduler.create_job_class(NULL, NULL);
CALL dbe_scheduler.create_program(NULL);
CALL dbe_scheduler.create_schedule(NULL, NULL);
CALL dbe_scheduler.define_program_argument(NULL, NULL, NULL);
CALL dbe_scheduler.disable(NULL, NULL);
CALL dbe_scheduler.disable_single(NULL, NULL);
CALL dbe_scheduler.drop_credential(NULL);
CALL dbe_scheduler.drop_job(NULL);
CALL dbe_scheduler.drop_job_class(NULL);
CALL dbe_scheduler.drop_program(NULL);
CALL dbe_scheduler.drop_schedule(NULL);
CALL dbe_scheduler.drop_single_job(NULL, NULL);
CALL dbe_scheduler.drop_single_job_class(NULL, NULL);
CALL dbe_scheduler.drop_single_program(NULL, NULL);
CALL dbe_scheduler.drop_single_schedule(NULL, NULL);
CALL dbe_scheduler.enable(NULL, NULL);
CALL dbe_scheduler.enable_single(NULL);
CALL dbe_scheduler.eval_calendar_string(NULL, NULL);
CALL dbe_scheduler.evaluate_calendar_string(NULL, NULL);
CALL dbe_scheduler.generate_job_name(NULL);
CALL dbe_scheduler.grant_user_authorization(NULL, NULL);
CALL dbe_scheduler.revoke_user_authorization(NULL, NULL);
CALL dbe_scheduler.run_backend_job(NULL, NULL);
CALL dbe_scheduler.run_foreground_job(NULL, NULL);
CALL dbe_scheduler.run_job(NULL, NULL);
CALL dbe_scheduler.set_attribute(NULL, NULL, NULL);
CALL dbe_scheduler.set_job_argument_value(NULL, NULL, NULL);
CALL dbe_scheduler.stop_job(NULL, NULL);
CALL dbe_scheduler.stop_single_job(NULL, NULL);

-- ============================================================
-- Domain: DbeSession  (2 functions)
-- ============================================================
CALL dbe_session.clear_context(NULL, NULL);
CALL dbe_session.set_context(NULL, NULL, NULL);

-- ============================================================
-- Domain: DbeSql  (63 functions)
-- ============================================================
CALL dbe_sql.bind_variable(NULL, NULL, NULL);
CALL dbe_sql.close_cursor(NULL);
CALL dbe_sql.column_value(NULL, NULL, NULL);
CALL dbe_sql.dbe_sql_get_result_char(NULL, NULL);
CALL dbe_sql.dbe_sql_get_result_long(NULL, NULL);
CALL dbe_sql.dbe_sql_get_result_raw(NULL, NULL, NULL);
CALL dbe_sql.describe_columns(NULL, NULL, NULL);
CALL dbe_sql.execute(NULL, NULL);
CALL dbe_sql.fetch_rows(NULL);
CALL dbe_sql.get_array_result_char(NULL, NULL, NULL);
CALL dbe_sql.get_array_result_int(NULL, NULL, NULL);
CALL dbe_sql.get_array_result_raw(NULL, NULL, NULL);
CALL dbe_sql.get_array_result_text(NULL, NULL, NULL);
CALL dbe_sql.get_result(NULL, NULL, NULL);
CALL dbe_sql.get_result_bytea(NULL, NULL);
CALL dbe_sql.get_result_char(NULL, NULL, NULL);
CALL dbe_sql.get_result_int(NULL, NULL);
CALL dbe_sql.get_result_long(NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.get_result_raw(NULL, NULL, NULL);
CALL dbe_sql.get_result_text(NULL, NULL);
CALL dbe_sql.get_result_unknown(NULL, NULL, NULL);
CALL dbe_sql.get_results(NULL, NULL, NULL);
CALL dbe_sql.get_results_bytea(NULL, NULL, NULL);
CALL dbe_sql.get_results_char(NULL, NULL, NULL);
CALL dbe_sql.get_results_int(NULL, NULL, NULL);
CALL dbe_sql.get_results_raw(NULL, NULL, NULL);
CALL dbe_sql.get_results_text(NULL, NULL, NULL);
CALL dbe_sql.get_variable_result(NULL, NULL, NULL);
CALL dbe_sql.get_variable_result_char(NULL, NULL);
CALL dbe_sql.get_variable_result_int(NULL, NULL, NULL);
CALL dbe_sql.get_variable_result_raw(NULL, NULL, NULL);
CALL dbe_sql.get_variable_result_text(NULL, NULL);
CALL dbe_sql.is_active(NULL);
CALL dbe_sql.last_row_count();
CALL dbe_sql.next_row(NULL);
CALL dbe_sql.open_cursor();
CALL dbe_sql.register_context();
CALL dbe_sql.register_variable(NULL, NULL, NULL);
CALL dbe_sql.run_and_next(NULL);
CALL dbe_sql.set_result_type(NULL, NULL, NULL);
CALL dbe_sql.set_result_type_bytea(NULL, NULL, NULL, NULL);
CALL dbe_sql.set_result_type_byteas(NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.set_result_type_char(NULL, NULL, NULL, NULL);
CALL dbe_sql.set_result_type_chars(NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.set_result_type_int(NULL, NULL);
CALL dbe_sql.set_result_type_ints(NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.set_result_type_long(NULL, NULL);
CALL dbe_sql.set_result_type_raw(NULL, NULL, NULL, NULL);
CALL dbe_sql.set_result_type_raws(NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.set_result_type_text(NULL, NULL, NULL);
CALL dbe_sql.set_result_type_texts(NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.set_result_type_unknown(NULL, NULL, NULL);
CALL dbe_sql.set_results_type(NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.sql_bind_array(NULL, NULL, NULL);
CALL dbe_sql.sql_bind_variable(NULL, NULL, NULL);
CALL dbe_sql.sql_describe_columns(NULL, NULL, NULL);
CALL dbe_sql.sql_get_tableof_values_c(NULL, NULL, NULL, NULL);
CALL dbe_sql.sql_get_values_c(NULL, NULL, NULL, NULL);
CALL dbe_sql.sql_run(NULL);
CALL dbe_sql.sql_set_results_type_c(NULL, NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.sql_set_sql(NULL, NULL, NULL);
CALL dbe_sql.sql_set_tableof_results_type_c(NULL, NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_sql.sql_unregister_context(NULL);

-- ============================================================
-- Domain: DbeSqlUtil  (6 functions)
-- ============================================================
CALL dbe_sql_util.create_abort_sql_patch(NULL, NULL);
CALL dbe_sql_util.create_hint_sql_patch(NULL, NULL);
CALL dbe_sql_util.disable_sql_patch(NULL);
CALL dbe_sql_util.drop_sql_patch(NULL);
CALL dbe_sql_util.enable_sql_patch(NULL);
CALL dbe_sql_util.show_sql_patch(NULL);

-- ============================================================
-- Domain: DbeStats  (15 functions)
-- ============================================================
CALL dbe_stats.get_stats_history_availability();
CALL dbe_stats.get_stats_history_retention();
CALL dbe_stats.lock_column_stats(NULL, NULL, NULL);
CALL dbe_stats.lock_partition_stats(NULL, NULL, NULL);
CALL dbe_stats.lock_schema_stats(NULL);
CALL dbe_stats.lock_table_stats(NULL);
CALL dbe_stats.purge_stats(NULL);
CALL dbe_stats.restore_column_stats(NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_stats.restore_partition_stats(NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_stats.restore_schema_stats(NULL, NULL, NULL, NULL);
CALL dbe_stats.restore_table_stats(NULL, NULL, NULL, NULL, NULL);
CALL dbe_stats.unlock_column_stats(NULL, NULL, NULL);
CALL dbe_stats.unlock_partition_stats(NULL, NULL, NULL);
CALL dbe_stats.unlock_schema_stats(NULL);
CALL dbe_stats.unlock_table_stats(NULL);

-- ============================================================
-- Domain: DbeTask  (11 functions)
-- ============================================================
CALL dbe_task.cancel(NULL);
CALL dbe_task.change(NULL, NULL);
CALL dbe_task.content(NULL, NULL);
CALL dbe_task.finish(NULL, NULL);
CALL dbe_task.id_submit(NULL, NULL);
CALL dbe_task.interval(NULL, NULL);
CALL dbe_task.job_submit(NULL, NULL);
CALL dbe_task.next_time(NULL, NULL);
CALL dbe_task.run(NULL, NULL);
CALL dbe_task.submit(NULL, NULL);
CALL dbe_task.update(NULL, NULL, NULL, NULL);

-- ============================================================
-- Domain: DbeUtility  (22 functions)
-- ============================================================
CALL dbe_utility.canonicalize(NULL, NULL);
CALL dbe_utility.comma_to_table(NULL, NULL, NULL);
CALL dbe_utility.compile_schema(NULL, NULL);
CALL dbe_utility.db_version(NULL);
CALL dbe_utility.exec_ddl_statement(NULL);
CALL dbe_utility.expand_sql_text_proc(NULL, NULL);
CALL dbe_utility.format_call_stack();
CALL dbe_utility.format_error_backtrace();
CALL dbe_utility.format_error_stack();
CALL dbe_utility.get_cpu_time();
CALL dbe_utility.get_endianness();
CALL dbe_utility.get_hash_value(NULL, NULL, NULL);
CALL dbe_utility.get_sql_hash(NULL, NULL, NULL);
CALL dbe_utility.get_sql_hash_func(NULL, NULL, NULL);
CALL dbe_utility.get_time();
CALL dbe_utility.is_bit_set(NULL, NULL);
CALL dbe_utility.is_cluster_database();
CALL dbe_utility.name_resolve(NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_utility.name_tokenize(NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_utility.old_current_schema();
CALL dbe_utility.old_current_user();
CALL dbe_utility.table_to_comma(NULL, NULL, NULL);

-- ============================================================
-- Domain: DbeXmlDom  (36 functions)
-- GaussDB commercial only
-- ============================================================
CALL dbe_xmldom.appendchild(NULL, NULL);
CALL dbe_xmldom.createelement(NULL, NULL);
CALL dbe_xmldom.createtextnode(NULL, NULL);
CALL dbe_xmldom.freedocument(NULL);
CALL dbe_xmldom.freeelement(NULL);
CALL dbe_xmldom.freenode(NULL);
CALL dbe_xmldom.freenodelist(NULL);
CALL dbe_xmldom.getattribute(NULL, NULL);
CALL dbe_xmldom.getattributes(NULL);
CALL dbe_xmldom.getchildnodes(NULL);
CALL dbe_xmldom.getchildrenbytagname(NULL, NULL);
CALL dbe_xmldom.getdocumentelement(NULL);
CALL dbe_xmldom.getfirstchild(NULL);
CALL dbe_xmldom.getlastchild(NULL);
CALL dbe_xmldom.getlength(NULL);
CALL dbe_xmldom.getlocalname(NULL, NULL);
CALL dbe_xmldom.getnameditem(NULL, NULL);
CALL dbe_xmldom.getnextsibling(NULL);
CALL dbe_xmldom.getnodename(NULL);
CALL dbe_xmldom.getnodetype(NULL);
CALL dbe_xmldom.getnodevalue(NULL);
CALL dbe_xmldom.getparentnode(NULL);
CALL dbe_xmldom.gettagname(NULL);
CALL dbe_xmldom.haschildnodes(NULL);
CALL dbe_xmldom.importnode(NULL, NULL, NULL);
CALL dbe_xmldom.isnull(NULL);
CALL dbe_xmldom.item(NULL, NULL);
CALL dbe_xmldom.makeelement(NULL);
CALL dbe_xmldom.makenode(NULL);
CALL dbe_xmldom.newdomdocument(NULL);
CALL dbe_xmldom.setattribute(NULL, NULL);
CALL dbe_xmldom.setcharset(NULL, NULL);
CALL dbe_xmldom.setdoctype(NULL, NULL);
CALL dbe_xmldom.setnodevalue(NULL, NULL);
CALL dbe_xmldom.writetobuffer(NULL, NULL);
CALL dbe_xmldom.writetoclob(NULL, NULL);

-- ============================================================
-- Domain: DbeXmlParser  (7 functions)
-- GaussDB commercial only
-- ============================================================
CALL dbe_xmlparser.freeparser(NULL);
CALL dbe_xmlparser.getdocument(NULL);
CALL dbe_xmlparser.getvalidationmode(NULL);
CALL dbe_xmlparser.newparser();
CALL dbe_xmlparser.parsebuffer(NULL, NULL);
CALL dbe_xmlparser.parseclob(NULL, NULL);
CALL dbe_xmlparser.setvalidationmode(NULL, NULL);

-- ============================================================
-- Domain: DbmsLob  (4 functions)
-- ============================================================
CALL dbms_lob.append(NULL, NULL);
CALL dbms_lob.read(NULL, NULL, NULL);
CALL dbms_lob.substr(NULL, NULL);
CALL dbms_lob.write(NULL, NULL, NULL);

-- ============================================================
-- Domain: DbmsOutput  (4 functions)
-- ============================================================
CALL dbms_output.disable();
CALL dbms_output.enable(NULL);
CALL dbms_output.put(NULL);
CALL dbms_output.put_line(NULL);

-- ============================================================
-- Domain: DbmsScheduler  (3 functions)
-- ============================================================
CALL dbms_scheduler.create_job(NULL);
CALL dbms_scheduler.drop_job(NULL);
CALL dbms_scheduler.run_job(NULL, NULL);

-- ============================================================
-- Domain: DbmsSql  (5 functions)
-- ============================================================
CALL dbms_sql.close_cursor(NULL);
CALL dbms_sql.column_value(NULL, NULL, NULL);
CALL dbms_sql.execute(NULL, NULL);
CALL dbms_sql.fetch_rows(NULL);
CALL dbms_sql.open_cursor();

-- ============================================================
-- Domain: DbmsUtility  (2 functions)
-- ============================================================
CALL dbms_utility.format_error_backtrace();
CALL dbms_utility.get_time();

-- ============================================================
-- Domain: ExceptionContext  (3 functions)
-- ============================================================
SELECT pg_exception_context();
SELECT pg_exception_detail();
SELECT pg_exception_hint();

-- ============================================================
-- Domain: Geometric  (12 functions)
-- ============================================================
SELECT area(NULL);
SELECT center(NULL);
SELECT circle(NULL);
SELECT diameter(NULL);
SELECT point(NULL, NULL);
SELECT polygon(NULL);
SELECT radius(NULL);
SELECT st_buffer(NULL, NULL);
SELECT st_envelope(NULL);
SELECT st_makepoint(NULL, NULL);
SELECT st_setsrid(NULL, NULL);
SELECT width(NULL);

-- ============================================================
-- Domain: Hash  (2 functions)
-- ============================================================
SELECT crc32(NULL, NULL);
SELECT ora_hash(NULL, NULL);

-- ============================================================
-- Domain: Json  (62 functions)
-- ============================================================
SELECT array_to_json(NULL, NULL);
SELECT json(NULL);
SELECT json_agg(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT json_append(NULL, NULL);
SELECT json_array(NULL);
SELECT json_array_element(NULL, NULL);
SELECT json_array_element_text(NULL, NULL);
SELECT * FROM json_array_elements(NULL);  -- set-returning
SELECT json_array_length(NULL);
SELECT json_build_array(NULL);
SELECT json_build_object(NULL);
SELECT json_contains(NULL, NULL);
SELECT json_contains_path(NULL, NULL);
SELECT json_depth(NULL);
SELECT * FROM json_each(NULL);  -- set-returning
SELECT * FROM json_each_text(NULL);  -- set-returning
SELECT json_extract_path(NULL);
SELECT json_extract_path_text(NULL);
SELECT json_keys(NULL, NULL);
SELECT json_length(NULL, NULL);
SELECT json_merge(NULL, NULL);
SELECT json_object(NULL);
SELECT json_object_agg(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT json_object_field(NULL, NULL);
SELECT json_object_field_text(NULL, NULL);
SELECT * FROM json_object_keys(NULL);  -- set-returning
SELECT json_quote(NULL);
SELECT json_remove(NULL, NULL);
SELECT json_replace(NULL, NULL);
SELECT json_search(NULL, NULL);
SELECT json_set(NULL, NULL);
SELECT json_type(NULL);
SELECT json_typeof(NULL);
SELECT json_unquote(NULL);
SELECT json_valid(NULL);
SELECT jsonb_agg(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT * FROM jsonb_array_elements(NULL);  -- set-returning
SELECT jsonb_array_length(NULL);
SELECT jsonb_build_array(NULL);
SELECT jsonb_build_object(NULL);
SELECT jsonb_cmp(NULL, NULL);
SELECT jsonb_contained(NULL, NULL);
SELECT jsonb_contains(NULL, NULL);
SELECT * FROM jsonb_each(NULL);  -- set-returning
SELECT * FROM jsonb_each_text(NULL);  -- set-returning
SELECT jsonb_eq(NULL, NULL);
SELECT jsonb_exists(NULL, NULL);
SELECT jsonb_exists_all(NULL, NULL);
SELECT jsonb_exists_any(NULL, NULL);
SELECT jsonb_ge(NULL, NULL);
SELECT jsonb_gt(NULL, NULL);
SELECT jsonb_hash(NULL);
SELECT jsonb_le(NULL, NULL);
SELECT jsonb_lt(NULL, NULL);
SELECT jsonb_ne(NULL, NULL);
SELECT * FROM jsonb_object_keys(NULL);  -- set-returning
SELECT jsonb_pretty(NULL);
SELECT jsonb_set(NULL, NULL, NULL);
SELECT jsonb_typeof(NULL);
SELECT row_to_json(NULL, NULL);
SELECT to_json(NULL);
SELECT to_jsonb(NULL);

-- ============================================================
-- Domain: Math  (35 functions)
-- ============================================================
SELECT abs(NULL);
SELECT acos(NULL);
SELECT asin(NULL);
SELECT atan(NULL);
SELECT atan2(NULL, NULL);
SELECT cbrt(NULL);
SELECT ceil(NULL);
SELECT ceiling(NULL);
SELECT cos(NULL);
SELECT cot(NULL);
SELECT degrees(NULL);
SELECT div(NULL, NULL);
SELECT exp(NULL);
SELECT factorial(NULL);
SELECT floor(NULL);
SELECT gcd(NULL, NULL);
SELECT lcm(NULL, NULL);
SELECT ln(NULL);
SELECT log(NULL, NULL);
SELECT log10(NULL);
SELECT mod(NULL, NULL);
SELECT pi();
SELECT power(NULL, NULL);
SELECT radians(NULL);
SELECT rand(NULL);
SELECT random();
SELECT remainder(NULL, NULL);
SELECT round(NULL, NULL);
SELECT setseed(NULL);
SELECT sign(NULL);
SELECT sin(NULL);
SELECT sqrt(NULL);
SELECT tan(NULL);
SELECT trunc(NULL, NULL);
SELECT width_bucket(NULL, NULL, NULL);

-- ============================================================
-- Domain: Network  (8 functions)
-- ============================================================
SELECT abbrev(NULL);
SELECT broadcast(NULL);
SELECT family(NULL);
SELECT host(NULL);
SELECT hostmask(NULL);
SELECT masklen(NULL);
SELECT netmask(NULL);
SELECT network(NULL);

-- ============================================================
-- Domain: OracleCompat  (17 functions)
-- ============================================================
SELECT add_months(NULL, NULL);
SELECT decode(NULL, NULL);
SELECT ifnull(NULL, NULL);
SELECT last_day(NULL);
SELECT months_between(NULL, NULL);
SELECT nanvl(NULL, NULL);
SELECT next_day(NULL, NULL);
SELECT nls_initcap(NULL, NULL);
SELECT nls_lower(NULL, NULL);
SELECT nls_sort(NULL, NULL);
SELECT nls_upper(NULL, NULL);
SELECT nlssort(NULL, NULL);
SELECT nvl(NULL, NULL);
SELECT nvl2(NULL, NULL, NULL);
SELECT rownum();
SELECT sys_connect_by_path(NULL, NULL);
SELECT sysdate();

-- ============================================================
-- Domain: Other  (15 functions)
-- ============================================================
SELECT coalesce(NULL, NULL);
CALL dbe_compression.get_compression_ratio(NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
CALL dbe_compression.get_compression_type(NULL, NULL, NULL, NULL);
CALL dbe_heat_map.row_heat_map(NULL, NULL, NULL);
CALL dbe_ilm.execute_ilm(NULL, NULL, NULL);
CALL dbe_ilm.stop_ilm(NULL, NULL);
CALL dbe_ilm_admin.customize_ilm(NULL, NULL);
CALL dbe_ilm_admin.disable_ilm();
CALL dbe_ilm_admin.enable_ilm();
SELECT empty_blob();
SELECT empty_clob();
SELECT * FROM generate_series(NULL, NULL);  -- set-returning
SELECT greatest(NULL, NULL);
SELECT least(NULL, NULL);
SELECT nullif(NULL, NULL);

-- ============================================================
-- Domain: PkgService  (18 functions)
-- ============================================================
CALL pkg_service.isubmit_on_nodes(NULL, NULL, NULL, NULL, NULL);
CALL pkg_service.job_cancel(NULL);
CALL pkg_service.job_finish(NULL, NULL);
CALL pkg_service.job_submit(NULL, NULL, NULL);
CALL pkg_service.job_update(NULL, NULL, NULL, NULL);
CALL pkg_service.sql_cancel(NULL);
CALL pkg_service.sql_clean_all_contexts();
CALL pkg_service.sql_get_array_result(NULL, NULL, NULL, NULL);
CALL pkg_service.sql_get_value(NULL, NULL, NULL);
CALL pkg_service.sql_get_variable_result(NULL, NULL, NULL);
CALL pkg_service.sql_is_context_active(NULL);
CALL pkg_service.sql_next_row(NULL);
CALL pkg_service.sql_register_context();
CALL pkg_service.sql_run(NULL);
CALL pkg_service.sql_set_result_type(NULL, NULL, NULL, NULL);
CALL pkg_service.sql_set_sql(NULL, NULL, NULL);
CALL pkg_service.sql_unregister_context(NULL);
CALL pkg_service.submit_on_nodes(NULL, NULL, NULL, NULL, NULL);

-- ============================================================
-- Domain: PkgUtil  (71 functions)
-- GaussDB commercial only (PKG_UTIL advanced package)
-- ============================================================
CALL pkg_util.app_read_action(NULL);
CALL pkg_util.app_read_client_info(NULL);
CALL pkg_util.app_read_module(NULL);
CALL pkg_util.app_set_action(NULL);
CALL pkg_util.app_set_client_info(NULL);
CALL pkg_util.app_set_module(NULL);
CALL pkg_util.bfile_close(NULL);
CALL pkg_util.bfile_get_length(NULL);
CALL pkg_util.bfile_open(NULL, NULL);
CALL pkg_util.blob_reset(NULL, NULL);
CALL pkg_util.clob_reset(NULL, NULL);
CALL pkg_util.exception_report_error(NULL, NULL);
CALL pkg_util.file_block_size(NULL);
CALL pkg_util.file_close_all();
CALL pkg_util.file_exists(NULL);
CALL pkg_util.file_getpos(NULL);
CALL pkg_util.file_is_close(NULL);
CALL pkg_util.file_newline(NULL);
CALL pkg_util.file_open(NULL, NULL);
CALL pkg_util.file_read(NULL, NULL);
CALL pkg_util.file_read_raw(NULL, NULL);
CALL pkg_util.file_readline(NULL, NULL);
CALL pkg_util.file_remove(NULL);
CALL pkg_util.file_rename(NULL, NULL, NULL, NULL);
CALL pkg_util.file_seek(NULL, NULL);
CALL pkg_util.file_set_dirname(NULL);
CALL pkg_util.file_set_max_line_size(NULL);
CALL pkg_util.file_size(NULL);
CALL pkg_util.file_write(NULL, NULL);
CALL pkg_util.file_write_raw(NULL, NULL);
CALL pkg_util.file_writeline(NULL, NULL);
CALL pkg_util.gs_compile_schema(NULL, NULL);
CALL pkg_util.io_print(NULL, NULL);
CALL pkg_util.loadblobfromfile(NULL, NULL, NULL, NULL, NULL);
CALL pkg_util.loadclobfromfile(NULL, NULL, NULL, NULL, NULL);
CALL pkg_util.lob_append(NULL, NULL);
CALL pkg_util.lob_append_huge(NULL, NULL);
CALL pkg_util.lob_compare(NULL, NULL);
CALL pkg_util.lob_converttoblob(NULL, NULL, NULL, NULL, NULL);
CALL pkg_util.lob_converttoblob_huge(NULL, NULL, NULL, NULL, NULL);
CALL pkg_util.lob_converttoclob(NULL, NULL, NULL, NULL, NULL);
CALL pkg_util.lob_converttoclob_huge(NULL, NULL, NULL, NULL, NULL);
CALL pkg_util.lob_copy_huge(NULL, NULL, NULL);
CALL pkg_util.lob_get_length(NULL);
CALL pkg_util.lob_match(NULL, NULL, NULL);
CALL pkg_util.lob_rawtotext(NULL);
CALL pkg_util.lob_read(NULL, NULL, NULL, NULL);
CALL pkg_util.lob_read_huge(NULL, NULL, NULL, NULL);
CALL pkg_util.lob_reset(NULL, NULL, NULL);
CALL pkg_util.lob_texttoraw(NULL);
CALL pkg_util.lob_write(NULL, NULL, NULL);
CALL pkg_util.lob_write_huge(NULL, NULL, NULL, NULL);
CALL pkg_util.lob_writeappend_huge(NULL, NULL, NULL);
CALL pkg_util.match_edit_distance_similarity(NULL, NULL);
CALL pkg_util.modify_package_state(NULL);
CALL pkg_util.random_get_value();
CALL pkg_util.random_set_seed(NULL);
CALL pkg_util.raw_cast_from_binary_integer(NULL, NULL);
CALL pkg_util.raw_cast_from_varchar2(NULL);
CALL pkg_util.raw_cast_to_binary_integer(NULL, NULL);
CALL pkg_util.raw_cast_to_varchar2(NULL);
CALL pkg_util.raw_get_length(NULL);
CALL pkg_util.read_bfile_to_blob(NULL);
CALL pkg_util.session_clear_context(NULL, NULL, NULL);
CALL pkg_util.session_search_context(NULL, NULL);
CALL pkg_util.session_set_context(NULL, NULL, NULL);
CALL pkg_util.utility_compile_schema(NULL, NULL);
CALL pkg_util.utility_format_call_stack();
CALL pkg_util.utility_format_error_backtrace();
CALL pkg_util.utility_format_error_stack();
CALL pkg_util.utility_get_time();

-- ============================================================
-- Domain: Range  (2 functions)
-- ============================================================
SELECT numrange(NULL, NULL);
SELECT tsrange(NULL, NULL);

-- ============================================================
-- Domain: String  (65 functions)
-- ============================================================
SELECT ascii(NULL);
SELECT ascii2(NULL);
SELECT asciistr(NULL);
SELECT bit_length(NULL);
SELECT btrim(NULL, NULL);
SELECT char_length(NULL);
SELECT character_length(NULL);
SELECT chr(NULL);
SELECT concat(NULL, NULL);
SELECT concat_ws(NULL, NULL);
SELECT dump(NULL, NULL);
SELECT encode(NULL, NULL);
SELECT find_in_set(NULL, NULL);
SELECT format(NULL, NULL);
SELECT get_bit(NULL, NULL);
SELECT get_byte(NULL, NULL);
SELECT group_concat(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT initcap(NULL);
SELECT instr(NULL, NULL);
SELECT instrb(NULL, NULL);
SELECT left(NULL, NULL);
SELECT length(NULL, NULL);
SELECT lengthb(NULL);
SELECT listagg(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT lower(NULL);
SELECT lpad(NULL, NULL);
SELECT ltrim(NULL, NULL);
SELECT nchr(NULL);
SELECT octet_length(NULL);
SELECT overlay(NULL, NULL, NULL);
SELECT position(NULL, NULL);
SELECT quote_ident(NULL);
SELECT quote_literal(NULL);
SELECT quote_nullable(NULL);
SELECT regexp_count(NULL, NULL);
SELECT regexp_instr(NULL, NULL);
SELECT regexp_like(NULL, NULL);
SELECT regexp_matches(NULL, NULL);
SELECT regexp_replace(NULL, NULL);
SELECT regexp_split_to_array(NULL, NULL);
SELECT * FROM regexp_split_to_table(NULL, NULL);  -- set-returning
SELECT regexp_substr(NULL, NULL);
SELECT repeat(NULL, NULL);
SELECT replace(NULL, NULL);
SELECT reverse(NULL);
SELECT right(NULL, NULL);
SELECT rpad(NULL, NULL);
SELECT rtrim(NULL, NULL);
SELECT set_bit(NULL, NULL, NULL);
SELECT set_byte(NULL, NULL, NULL);
SELECT split_part(NULL, NULL, NULL);
SELECT string_agg(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT strpos(NULL, NULL);
SELECT substr(NULL, NULL);
SELECT substrb(NULL, NULL);
SELECT substring(NULL, NULL);
SELECT substring_index(NULL, NULL, NULL);
SELECT to_multi_byte(NULL);
SELECT to_single_byte(NULL);
SELECT translate(NULL, NULL, NULL);
SELECT trim(NULL, NULL);
SELECT unistr(NULL);
SELECT upper(NULL);
SELECT vsize(NULL);
SELECT wm_concat(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate

-- ============================================================
-- Domain: System  (112 functions)
-- ============================================================
SELECT col_description(NULL, NULL);
SELECT current_database();
SELECT current_schema(NULL);
SELECT current_setting(NULL, NULL);
SELECT current_user();
SELECT currval(NULL, NULL);
SELECT format_type(NULL, NULL);
SELECT has_schema_privilege(NULL, NULL);
SELECT has_table_privilege(NULL, NULL);
SELECT inet_client_addr();
SELECT inet_client_port();
SELECT inet_server_addr();
SELECT inet_server_port();
SELECT last_insert_id(NULL);
SELECT lastval();
SELECT nextval(NULL);
SELECT pg_advisory_lock(NULL, NULL);
SELECT pg_advisory_unlock(NULL, NULL);
SELECT pg_advisory_xact_lock(NULL, NULL);
SELECT pg_backend_pid();
SELECT pg_cancel_backend(NULL);
SELECT pg_collation_is_visible(NULL, NULL);
SELECT pg_column_size(NULL);
SELECT pg_conf_load_time();
SELECT pg_conversion_is_visible(NULL);
SELECT pg_create_logical_replication_slot(NULL, NULL);
SELECT pg_create_physical_replication_slot(NULL, NULL);
SELECT pg_current_xlog_location();
SELECT pg_database_size(NULL);
SELECT pg_describe_object(NULL, NULL, NULL);
SELECT pg_drop_replication_slot(NULL);
SELECT pg_export_snapshot();
SELECT pg_function_is_visible(NULL);
SELECT pg_get_constraintdef(NULL, NULL);
SELECT pg_get_expr(NULL, NULL);
SELECT pg_get_functiondef(NULL);
SELECT pg_get_indexdef(NULL, NULL);
SELECT pg_get_keywords();
SELECT pg_get_ruledef(NULL, NULL);
SELECT pg_get_serial_sequence(NULL, NULL);
SELECT pg_get_triggerdef(NULL);
SELECT pg_get_userbyid(NULL);
SELECT pg_get_viewdef(NULL, NULL);
SELECT pg_has_role(NULL, NULL);
SELECT pg_identify_object(NULL, NULL, NULL);
SELECT pg_indexes_size(NULL);
SELECT pg_is_in_recovery();
SELECT pg_is_other_temp_schema(NULL);
SELECT pg_last_xact_replay_timestamp();
SELECT pg_listening_channels();
SELECT pg_logical_slot_get_binary_changes(NULL, NULL);
SELECT pg_logical_slot_get_changes(NULL, NULL);
SELECT pg_logical_slot_peek_binary_changes(NULL, NULL);
SELECT pg_logical_slot_peek_changes(NULL, NULL);
SELECT pg_ls_dir(NULL);
SELECT pg_my_temp_schema();
SELECT pg_opclass_is_visible(NULL);
SELECT pg_operator_is_visible(NULL);
SELECT pg_opfamily_is_visible(NULL);
SELECT pg_postmaster_start_time();
SELECT pg_prepared_statement(NULL);
SELECT pg_prepared_xact(NULL);
SELECT pg_query_audit(NULL, NULL);
SELECT pg_read_binary_file(NULL, NULL);
SELECT pg_read_file(NULL, NULL);
SELECT pg_relation_filenode(NULL);
SELECT pg_relation_filepath(NULL);
SELECT pg_relation_size(NULL, NULL);
SELECT pg_replication_origin_create(NULL);
SELECT pg_replication_origin_drop(NULL);
SELECT pg_replication_origin_oid(NULL);
SELECT pg_replication_origin_progress(NULL, NULL);
SELECT pg_rotate_logfile();
SELECT pg_size_pretty(NULL);
SELECT pg_sleep(NULL);
SELECT pg_start_backup(NULL, NULL);
SELECT pg_stat_file(NULL);
SELECT pg_stop_backup(NULL);
SELECT pg_switch_xlog();
SELECT pg_table_is_visible(NULL, NULL);
SELECT pg_table_size(NULL);
SELECT pg_tablespace_databases(NULL);
SELECT pg_tablespace_location(NULL);
SELECT pg_tablespace_size(NULL);
SELECT pg_terminate_backend(NULL);
SELECT pg_total_relation_size(NULL);
SELECT pg_try_advisory_lock(NULL, NULL);
SELECT pg_try_advisory_xact_lock(NULL, NULL);
SELECT pg_ts_config_is_visible(NULL);
SELECT pg_ts_dict_is_visible(NULL);
SELECT pg_ts_parser_is_visible(NULL);
SELECT pg_ts_template_is_visible(NULL);
SELECT pg_type_is_visible(NULL);
SELECT pg_typeof(NULL);
SELECT pg_xlog_location_diff(NULL, NULL);
SELECT pg_xlog_replay_pause();
SELECT pg_xlog_replay_resume();
SELECT pg_xlogfile_name(NULL);
SELECT pg_xlogfile_name_offset(NULL);
SELECT session_user();
SELECT set_config(NULL, NULL);
SELECT setval(NULL, NULL);
SELECT sys_context(NULL, NULL);
SELECT txid_current();
SELECT txid_current_snapshot();
SELECT txid_snapshot_xip(NULL);
SELECT txid_snapshot_xmax(NULL);
SELECT txid_snapshot_xmin(NULL);
SELECT txid_visible_in_snapshot(NULL, NULL);
SELECT user();
SELECT uuid_short();
SELECT version();

-- ============================================================
-- Domain: TextSearch  (14 functions)
-- ============================================================
SELECT get_current_ts_config();
SELECT plainto_tsquery(NULL, NULL);
SELECT querytree(NULL);
SELECT to_tsquery(NULL, NULL);
SELECT to_tsvector(NULL, NULL);
SELECT ts_headline(NULL, NULL);
SELECT ts_lexize(NULL, NULL);
SELECT ts_parse(NULL, NULL);
SELECT ts_rank(NULL, NULL);
SELECT ts_rank_cd(NULL, NULL);
SELECT ts_rewrite(NULL, NULL);
SELECT ts_stat(NULL, NULL);
SELECT ts_token_type(NULL);
SELECT tsvector_update_trigger();

-- ============================================================
-- Domain: TypeConversion  (26 functions)
-- ============================================================
SELECT convert(NULL, NULL);
SELECT convert_from(NULL, NULL);
SELECT convert_to(NULL, NULL);
SELECT hextoraw(NULL);
SELECT intervaltonum(NULL);
SELECT numtoday(NULL);
SELECT numtodsinterval(NULL, NULL);
SELECT numtoyminterval(NULL, NULL);
SELECT rawout(NULL);
SELECT rawsend(NULL);
SELECT rawtohex(NULL);
SELECT rawtohex2(NULL);
SELECT to_ascii(NULL, NULL);
SELECT to_bigint(NULL);
SELECT to_binary_double(NULL, NULL);
SELECT to_binary_float(NULL, NULL);
SELECT to_blob(NULL);
SELECT to_char(NULL, NULL);
SELECT to_clob(NULL);
SELECT to_date(NULL, NULL);
SELECT to_dsinterval(NULL);
SELECT to_hex(NULL);
SELECT to_nchar(NULL, NULL);
SELECT to_number(NULL, NULL);
SELECT to_yminterval(NULL);
SELECT treat(NULL);

-- ============================================================
-- Domain: UtlFile  (5 functions)
-- ============================================================
CALL utl_file.fclose(NULL);
CALL utl_file.fclose_all();
CALL utl_file.fopen(NULL, NULL);
CALL utl_file.get_line(NULL, NULL);
CALL utl_file.put_line(NULL, NULL);

-- ============================================================
-- Domain: Window  (12 functions)
-- ============================================================
SELECT cume_dist() OVER ();  -- window
SELECT dense_rank() OVER ();  -- window
SELECT first_value(NULL) OVER ();  -- window
SELECT lag(NULL, NULL) OVER ();  -- window
SELECT last_value(NULL) OVER ();  -- window
SELECT lead(NULL, NULL) OVER ();  -- window
SELECT nth_value(NULL, NULL) OVER ();  -- window
SELECT ntile(NULL) OVER ();  -- window
SELECT percent_rank() OVER ();  -- window
SELECT rank() OVER ();  -- window
SELECT ratio_to_report(NULL) OVER ();  -- window
SELECT row_number() OVER ();  -- window

-- ============================================================
-- Domain: Xml  (44 functions)
-- ============================================================
SELECT createxml(NULL, NULL);
SELECT cursor_to_xml(NULL, NULL, NULL, NULL, NULL);
SELECT cursor_to_xmlschema(NULL, NULL, NULL, NULL, NULL);
SELECT database_to_xml(NULL, NULL, NULL, NULL);
SELECT database_to_xml_and_xmlschema(NULL, NULL, NULL, NULL);
SELECT database_to_xmlschema(NULL, NULL, NULL, NULL);
SELECT existsnode(NULL, NULL);
SELECT extractvalue(NULL, NULL);
SELECT extractxml(NULL, NULL);
SELECT getblobval(NULL);
SELECT getclobval(NULL);
SELECT getnamespace(NULL);
SELECT getnumberval(NULL);
SELECT getrootelement(NULL);
SELECT getstringval(NULL);
SELECT isfragment(NULL);
SELECT query_to_xml(NULL, NULL, NULL, NULL);
SELECT query_to_xml_and_xmlschema(NULL, NULL, NULL, NULL);
SELECT query_to_xmlschema(NULL, NULL, NULL, NULL);
SELECT schema_to_xml(NULL, NULL, NULL, NULL);
SELECT schema_to_xml_and_xmlschema(NULL, NULL, NULL, NULL);
SELECT schema_to_xmlschema(NULL, NULL, NULL, NULL);
SELECT table_to_xml(NULL, NULL, NULL, NULL);
SELECT table_to_xml_and_xmlschema(NULL, NULL, NULL, NULL);
SELECT table_to_xmlschema(NULL, NULL, NULL, NULL);
SELECT xml_is_well_formed(NULL);
SELECT xml_is_well_formed_content(NULL, NULL);
SELECT xml_is_well_formed_document(NULL, NULL);
SELECT xmlagg(DISTINCT v) FROM (VALUES (1)) AS t(v);  -- aggregate
SELECT xmlattributes(NULL);
SELECT xmlcomment(NULL);
SELECT xmlconcat(NULL);
SELECT xmlelement(NULL);
SELECT xmlexists(NULL, NULL);
SELECT xmlforest(NULL);
SELECT xmlparse(NULL, NULL);
SELECT xmlpi(NULL, NULL);
SELECT xmlquery(NULL, NULL);
SELECT xmlroot(NULL, NULL);
SELECT xmlsequence(NULL);
SELECT xmlserialize(NULL, NULL);
SELECT xmltype(NULL);
SELECT xpath(NULL, NULL);
SELECT xpath_exists(NULL, NULL);
