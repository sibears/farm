import * as React from 'react';
import ReportProblemIcon from '@mui/icons-material/ReportProblem';
import InfoIcon from '@mui/icons-material/Info';
import AutorenewIcon from '@mui/icons-material/Autorenew';
import DoneIcon from '@mui/icons-material/Done';
import Chip from '@mui/material/Chip';
import { styled } from '@mui/material/styles';

const StyledChip = styled(Chip)(({ theme }) => ({
  justifyContent: 'left',
  '& .icon': {
    color: 'inherit',
  },
  '&.SKIPPED': {
    color: (theme.vars || theme).palette.info.dark,
    border: `1px solid ${(theme.vars || theme).palette.info.main}`,
  },
  '&.ACCEPTED': {
    color: (theme.vars || theme).palette.success.dark,
    border: `1px solid ${(theme.vars || theme).palette.success.main}`,
  },
  '&.QUEUED': {
    color: (theme.vars || theme).palette.warning.dark,
    border: `1px solid ${(theme.vars || theme).palette.warning.main}`,
  },
  '&.REJECTED': {
    color: (theme.vars || theme).palette.error.dark,
    border: `1px solid ${(theme.vars || theme).palette.error.main}`,
  },
}));


const Status = React.memo((props) => {
  const { status } = props;

  let icon = null;
  if (status === 'REJECTED') {
    icon = <ReportProblemIcon className="icon" />;
  } else if (status === 'SKIPPED') { // Open
    icon = <InfoIcon className="icon" />;
  } else if (status === 'QUEUED') { // PartiallyFilled
    icon = <AutorenewIcon className="icon" />;
  } else if (status === 'ACCEPTED') { // Filled
    icon = <DoneIcon className="icon" />;
  }

  let label = status;

  return (
    <StyledChip className={status} icon={icon} size="small" label={label} variant="outlined" />
  );
});

export function renderStatus(params) {
  if (params.value == null) {
    return '';
  }

  return <Status status={params.value} />;
}