use flowy_derive::ProtoBuf;
use flowy_error::ErrorCode;
use flowy_grid_data_model::parser::{NotEmptyStr, NotEmptyUuid};

#[derive(ProtoBuf, Default)]
pub struct CreateSelectOptionPayload {
    #[pb(index = 1)]
    pub cell_identifier: CellIdentifierPayload,

    #[pb(index = 2)]
    pub option_name: String,
}

pub struct CreateSelectOptionParams {
    pub cell_identifier: CellIdentifier,
    pub option_name: String,
}

impl TryInto<CreateSelectOptionParams> for CreateSelectOptionPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<CreateSelectOptionParams, Self::Error> {
        let option_name = NotEmptyStr::parse(self.option_name).map_err(|_| ErrorCode::SelectOptionNameIsEmpty)?;
        let cell_identifier = self.cell_identifier.try_into()?;
        Ok(CreateSelectOptionParams {
            cell_identifier,
            option_name: option_name.0,
        })
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct CellIdentifierPayload {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub field_id: String,

    #[pb(index = 3)]
    pub row_id: String,
}

pub struct CellIdentifier {
    pub grid_id: String,
    pub field_id: String,
    pub row_id: String,
}

impl TryInto<CellIdentifier> for CellIdentifierPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<CellIdentifier, Self::Error> {
        let grid_id = NotEmptyUuid::parse(self.grid_id).map_err(|_| ErrorCode::GridIdIsEmpty)?;
        let field_id = NotEmptyUuid::parse(self.field_id).map_err(|_| ErrorCode::FieldIdIsEmpty)?;
        let row_id = NotEmptyUuid::parse(self.row_id).map_err(|_| ErrorCode::RowIdIsEmpty)?;
        Ok(CellIdentifier {
            grid_id: grid_id.0,
            field_id: field_id.0,
            row_id: row_id.0,
        })
    }
}

#[derive(ProtoBuf, Default)]
pub struct SelectOptionName {
    #[pb(index = 1)]
    pub name: String,
}
