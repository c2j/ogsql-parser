use crate::ast::*;
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn is_grant_role(&self) -> bool {
        if self.match_keyword(Keyword::ROLE) || self.match_keyword(Keyword::ROLES) {
            return true;
        }
        // If the next token is not a known privilege keyword and not ALL,
        // and the token after that is TO or comma, it's GRANT ROLE
        match self.peek() {
            Token::Keyword(kw) => {
                let kw_name = kw.as_str().to_uppercase();
                let is_priv = matches!(
                    kw_name.as_str(),
                    "SELECT"
                        | "INSERT"
                        | "UPDATE"
                        | "DELETE"
                        | "USAGE"
                        | "CREATE"
                        | "CONNECT"
                        | "TEMPORARY"
                        | "EXECUTE"
                        | "TRIGGER"
                        | "REFERENCES"
                        | "ALTER"
                        | "DROP"
                        | "COMMENT"
                        | "INDEX"
                        | "VACUUM"
                );
                if is_priv {
                    return false;
                }
                // ALL: GRANT ALL ON (privilege), GRANT ALL TO (role), GRANT ALL PRIVILEGES TO (role)
                if kw_name == "ALL" {
                    if self.tokens.len() > self.pos + 1 {
                        let next = &self.tokens[self.pos + 1].token;
                        if matches!(next, Token::Comma | Token::Keyword(Keyword::TO)) {
                            return true;
                        }
                        // ALL PRIVILEGES TO → role grant (no ON clause)
                        let is_privileges = match next {
                            Token::Ident(s) => s.to_uppercase() == "PRIVILEGES",
                            Token::Keyword(kw) => kw.as_str().eq_ignore_ascii_case("PRIVILEGES"),
                            _ => false,
                        };
                        if is_privileges && self.tokens.len() > self.pos + 2 {
                            let after_priv = &self.tokens[self.pos + 2].token;
                            if matches!(after_priv, Token::Keyword(Keyword::TO)) {
                                return true;
                            }
                        }
                    }
                    return false;
                }
                // Otherwise, look ahead: if followed by comma or TO, it's GRANT ROLE
                if self.tokens.len() > self.pos + 1 {
                    let next = &self.tokens[self.pos + 1].token;
                    matches!(next, Token::Comma | Token::Keyword(Keyword::TO))
                } else {
                    false
                }
            }
            Token::Ident(s) => {
                let upper = s.to_uppercase();
                if matches!(upper.as_str(), "USAGE") {
                    return false;
                }
                if self.tokens.len() > self.pos + 1 {
                    let next = &self.tokens[self.pos + 1].token;
                    matches!(next, Token::Comma | Token::Keyword(Keyword::TO))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub(crate) fn is_revoke_role(&self) -> bool {
        if self.match_keyword(Keyword::ROLE) || self.match_keyword(Keyword::ROLES) {
            return true;
        }
        match self.peek() {
            Token::Keyword(kw) => {
                let kw_name = kw.as_str().to_uppercase();
                let is_priv = matches!(
                    kw_name.as_str(),
                    "SELECT"
                        | "INSERT"
                        | "UPDATE"
                        | "DELETE"
                        | "USAGE"
                        | "CREATE"
                        | "CONNECT"
                        | "TEMPORARY"
                        | "EXECUTE"
                        | "TRIGGER"
                        | "REFERENCES"
                        | "ALTER"
                        | "DROP"
                        | "COMMENT"
                        | "INDEX"
                        | "VACUUM"
                );
                if is_priv {
                    return false;
                }
                if kw_name == "ALL" {
                    if self.tokens.len() > self.pos + 1 {
                        let next = &self.tokens[self.pos + 1].token;
                        if matches!(next, Token::Comma | Token::Keyword(Keyword::FROM)) {
                            return true;
                        }
                        let is_privileges = match next {
                            Token::Ident(s) => s.to_uppercase() == "PRIVILEGES",
                            Token::Keyword(kw) => kw.as_str().eq_ignore_ascii_case("PRIVILEGES"),
                            _ => false,
                        };
                        if is_privileges && self.tokens.len() > self.pos + 2 {
                            let after_priv = &self.tokens[self.pos + 2].token;
                            if matches!(after_priv, Token::Keyword(Keyword::FROM)) {
                                return true;
                            }
                        }
                    }
                    return false;
                }
                if self.tokens.len() > self.pos + 1 {
                    let next = &self.tokens[self.pos + 1].token;
                    matches!(next, Token::Comma | Token::Keyword(Keyword::FROM))
                } else {
                    false
                }
            }
            Token::Ident(s) => {
                let upper = s.to_uppercase();
                if matches!(upper.as_str(), "USAGE") {
                    return false;
                }
                if self.tokens.len() > self.pos + 1 {
                    let next = &self.tokens[self.pos + 1].token;
                    matches!(next, Token::Comma | Token::Keyword(Keyword::FROM))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub(crate) fn parse_grant_role(&mut self) -> Result<GrantRoleStatement, ParserError> {
        if self.match_keyword(Keyword::ROLE) || self.match_keyword(Keyword::ROLES) {
            self.advance();
        }
        let mut roles = vec![self.parse_role_identifier()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            roles.push(self.parse_role_identifier()?);
        }
        self.expect_keyword(Keyword::TO)?;
        let mut grantees = vec![self.parse_identifier()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            grantees.push(self.parse_identifier()?);
        }
        let mut with_admin_option = false;
        let mut granted_by = None;
        if self.try_consume_keyword(Keyword::WITH) {
            if self.match_keyword(Keyword::ADMIN) || self.match_ident_str("ADMIN") {
                self.advance();
                self.expect_keyword(Keyword::OPTION)?;
                with_admin_option = true;
            }
        }
        if self.try_consume_keyword(Keyword::GRANTED) {
            self.expect_keyword(Keyword::BY)?;
            granted_by = Some(self.parse_identifier()?);
        }
        Ok(GrantRoleStatement {
            roles,
            grantees,
            with_admin_option,
            granted_by,
        })
    }

    pub(crate) fn parse_revoke_role(&mut self) -> Result<RevokeRoleStatement, ParserError> {
        if self.match_keyword(Keyword::ROLE) || self.match_keyword(Keyword::ROLES) {
            self.advance();
        }
        let mut roles = vec![self.parse_role_identifier()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            roles.push(self.parse_role_identifier()?);
        }
        self.expect_keyword(Keyword::FROM)?;
        let mut grantees = vec![self.parse_identifier()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            grantees.push(self.parse_identifier()?);
        }
        let mut granted_by = None;
        if self.try_consume_keyword(Keyword::GRANTED) {
            self.expect_keyword(Keyword::BY)?;
            granted_by = Some(self.parse_identifier()?);
        }
        let cascade = self.try_consume_keyword(Keyword::CASCADE);
        Ok(RevokeRoleStatement {
            roles,
            grantees,
            granted_by,
            cascade,
        })
    }

    pub(crate) fn parse_grant(&mut self) -> Result<GrantStatement, ParserError> {
        let mut privileges = Vec::new();
        let target;
        let grantees;
        let mut with_grant_option = false;
        let mut granted_by = None;

        privileges = self.parse_privileges()?;

        self.expect_keyword(Keyword::ON)?;

        target = self.parse_grant_target()?;

        self.expect_keyword(Keyword::TO)?;

        grantees = self.parse_grantee_list()?;

        if self.try_consume_keyword(Keyword::WITH) {
            if self.match_keyword(Keyword::GRANT) {
                self.advance();
                self.expect_keyword(Keyword::OPTION)?;
                with_grant_option = true;
            }
        }

        if self.try_consume_keyword(Keyword::GRANTED) {
            self.expect_keyword(Keyword::BY)?;
            granted_by = Some(self.parse_identifier()?);
        }

        Ok(GrantStatement {
            privileges,
            target,
            grantees,
            with_grant_option,
            granted_by,
        })
    }

    pub(crate) fn parse_revoke(&mut self) -> Result<RevokeStatement, ParserError> {
        let mut privileges = Vec::new();
        let target;
        let grantees;
        let mut cascade = false;
        let mut granted_by = None;

        privileges = self.parse_privileges()?;

        self.expect_keyword(Keyword::ON)?;

        target = self.parse_grant_target()?;

        self.expect_keyword(Keyword::FROM)?;

        grantees = self.parse_grantee_list()?;

        if self.match_keyword(Keyword::CASCADE) {
            self.advance();
            cascade = true;
        } else {
            self.try_consume_keyword(Keyword::RESTRICT);
        }

        if self.try_consume_keyword(Keyword::GRANTED) {
            self.expect_keyword(Keyword::BY)?;
            granted_by = Some(self.parse_identifier()?);
        }

        Ok(RevokeStatement {
            privileges,
            target,
            grantees,
            cascade,
            granted_by,
        })
    }

    fn parse_privileges(&mut self) -> Result<Vec<Privilege>, ParserError> {
        let mut privileges = Vec::new();

        if self.match_keyword(Keyword::ALL) {
            self.advance();
            self.try_consume_keyword(Keyword::PRIVILEGES);
            privileges.push(Privilege::All);
            return Ok(privileges);
        }

        loop {
            let priv_kind = match self.peek_keyword() {
                Some(Keyword::SELECT) => {
                    self.advance();
                    if self.match_token(&Token::LParen) {
                        self.advance();
                        let mut cols = vec![self.parse_identifier()?];
                        while self.match_token(&Token::Comma) {
                            self.advance();
                            cols.push(self.parse_identifier()?);
                        }
                        self.expect_token(&Token::RParen)?;
                        Privilege::SelectColumns(cols)
                    } else {
                        Privilege::Select
                    }
                }
                Some(Keyword::INSERT) => {
                    self.advance();
                    Privilege::Insert
                }
                Some(Keyword::UPDATE) => {
                    self.advance();
                    if self.match_token(&Token::LParen) {
                        self.advance();
                        let mut cols = vec![self.parse_identifier()?];
                        while self.match_token(&Token::Comma) {
                            self.advance();
                            cols.push(self.parse_identifier()?);
                        }
                        self.expect_token(&Token::RParen)?;
                        Privilege::UpdateColumns(cols)
                    } else {
                        Privilege::Update
                    }
                }
                Some(Keyword::DELETE_P) => {
                    self.advance();
                    Privilege::Delete
                }
                Some(Keyword::CREATE) => {
                    self.advance();
                    Privilege::Create
                }
                Some(Keyword::CONNECT) => {
                    self.advance();
                    Privilege::Connect
                }
                Some(Keyword::TEMPORARY) | Some(Keyword::TEMP) => {
                    self.advance();
                    Privilege::Temporary
                }
                Some(Keyword::EXECUTE) => {
                    self.advance();
                    Privilege::Execute
                }
                Some(Keyword::TRIGGER) => {
                    self.advance();
                    Privilege::Trigger
                }
                Some(Keyword::REFERENCES) => {
                    self.advance();
                    Privilege::References
                }
                Some(Keyword::ALTER) => {
                    self.advance();
                    Privilege::Alter
                }
                Some(Keyword::DROP) => {
                    self.advance();
                    Privilege::Drop
                }
                Some(Keyword::COMMENT) => {
                    self.advance();
                    Privilege::Comment
                }
                Some(Keyword::INDEX) => {
                    self.advance();
                    Privilege::Index
                }
                Some(Keyword::VACUUM) => {
                    self.advance();
                    Privilege::Vacuum
                }
                _ => {
                    if let Token::Ident(s) = self.peek() {
                        let name = s.to_uppercase();
                        match name.as_str() {
                            "USAGE" => {
                                self.advance();
                                privileges.push(Privilege::Usage);
                            }
                            _ => {
                                return Err(ParserError::UnexpectedToken {
                                    location: self.current_location(),
                                    expected: "privilege keyword".to_string(),
                                    got: name,
                                });
                            }
                        }
                        if self.match_token(&Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                        continue;
                    } else {
                        return Err(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "privilege keyword".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                    }
                }
            };
            privileges.push(priv_kind);

            if self.match_token(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(privileges)
    }

    fn parse_grant_target(&mut self) -> Result<GrantTarget, ParserError> {
        if self.match_keyword(Keyword::ALL) {
            self.advance();
            let what = match self.peek_keyword() {
                Some(Keyword::TABLES) => {
                    self.advance();
                    self.expect_keyword(Keyword::IN_P)?;
                    self.expect_keyword(Keyword::SCHEMA)?;
                    let mut schemas = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        schemas.push(self.parse_identifier()?);
                    }
                    return Ok(GrantTarget::AllTablesInSchema(schemas));
                }
                Some(Keyword::FUNCTIONS) => {
                    self.advance();
                    self.expect_keyword(Keyword::IN_P)?;
                    self.expect_keyword(Keyword::SCHEMA)?;
                    let mut schemas = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        schemas.push(self.parse_identifier()?);
                    }
                    return Ok(GrantTarget::AllFunctionsInSchema(schemas));
                }
                Some(Keyword::SEQUENCES) => {
                    self.advance();
                    self.expect_keyword(Keyword::IN_P)?;
                    self.expect_keyword(Keyword::SCHEMA)?;
                    let mut schemas = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        schemas.push(self.parse_identifier()?);
                    }
                    return Ok(GrantTarget::AllSequencesInSchema(schemas));
                }
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "TABLES | FUNCTIONS | SEQUENCES".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
            };
        }
        match self.peek_keyword() {
            Some(Keyword::TABLE) => {
                self.advance();
                let mut tables = Vec::new();
                tables.push(self.parse_object_name()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    tables.push(self.parse_object_name()?);
                }
                Ok(GrantTarget::Table(tables))
            }
            Some(Keyword::SEQUENCE) => {
                self.advance();
                let mut seqs = Vec::new();
                seqs.push(self.parse_object_name()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    seqs.push(self.parse_object_name()?);
                }
                Ok(GrantTarget::Sequence(seqs))
            }
            Some(Keyword::DATABASE) => {
                self.advance();
                let mut dbs = Vec::new();
                dbs.push(self.parse_identifier()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    dbs.push(self.parse_identifier()?);
                }
                Ok(GrantTarget::Database(dbs))
            }
            Some(Keyword::SCHEMA) => {
                self.advance();
                let mut schemas = Vec::new();
                schemas.push(self.parse_identifier()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    schemas.push(self.parse_identifier()?);
                }
                Ok(GrantTarget::Schema(schemas))
            }
            Some(Keyword::TABLESPACE) => {
                self.advance();
                let mut tbs = Vec::new();
                tbs.push(self.parse_identifier()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    tbs.push(self.parse_identifier()?);
                }
                Ok(GrantTarget::Tablespace(tbs))
            }
            Some(Keyword::FUNCTION) | Some(Keyword::PROCEDURE) => {
                self.advance();
                let mut funcs = Vec::new();
                funcs.push(self.parse_object_name()?);
                if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut depth = 1;
                    while depth > 0 {
                        match self.peek() {
                            Token::LParen => {
                                depth += 1;
                                self.advance();
                            }
                            Token::RParen => {
                                depth -= 1;
                                self.advance();
                            }
                            Token::Eof => break,
                            _ => self.advance(),
                        }
                    }
                }
                while self.match_token(&Token::Comma) {
                    self.advance();
                    funcs.push(self.parse_object_name()?);
                    if self.match_token(&Token::LParen) {
                        self.advance();
                        let mut depth = 1;
                        while depth > 0 {
                            match self.peek() {
                                Token::LParen => {
                                    depth += 1;
                                    self.advance();
                                }
                                Token::RParen => {
                                    depth -= 1;
                                    self.advance();
                                }
                                Token::Eof => break,
                                _ => self.advance(),
                            }
                        }
                    }
                }
                Ok(GrantTarget::Function(funcs))
            }
            Some(Keyword::LANGUAGE) => {
                self.advance();
                let mut langs = Vec::new();
                langs.push(self.parse_identifier()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    langs.push(self.parse_identifier()?);
                }
                Ok(GrantTarget::Language(langs))
            }
            Some(Keyword::LARGE_P) => {
                self.advance();
                self.expect_keyword(Keyword::OBJECT_P)?;
                let mut oids = Vec::new();
                oids.push(self.parse_identifier()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    oids.push(self.parse_identifier()?);
                }
                Ok(GrantTarget::LargeObject(oids))
            }
            Some(Keyword::TYPE_P) => {
                self.advance();
                let mut types = Vec::new();
                types.push(self.parse_object_name()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    types.push(self.parse_object_name()?);
                }
                Ok(GrantTarget::Type(types))
            }
            _ => {
                let mut tables = Vec::new();
                tables.push(self.parse_object_name()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    tables.push(self.parse_object_name()?);
                }
                Ok(GrantTarget::Table(tables))
            }
        }
    }

    fn parse_grantee_list(&mut self) -> Result<Vec<String>, ParserError> {
        let mut grantees = Vec::new();
        grantees.push(self.parse_identifier()?);
        while self.match_token(&Token::Comma) {
            self.advance();
            grantees.push(self.parse_identifier()?);
        }
        Ok(grantees)
    }

    fn parse_role_identifier(&mut self) -> Result<String, ParserError> {
        if self.match_keyword(Keyword::ALL) {
            self.advance();
            let name = "ALL".to_string();
            if self.match_ident_str("PRIVILEGES") {
                self.advance();
                return Ok(format!("{} PRIVILEGES", name));
            }
            return Ok(name);
        }
        let name = self.parse_identifier()?;
        Ok(name)
    }

    // ── Wave 8: CREATE TRIGGER + MATERIALIZED VIEW ──
}
