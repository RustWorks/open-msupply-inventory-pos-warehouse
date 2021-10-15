import React, { ChangeEvent, useState } from 'react';
import { ComponentStory, ComponentMeta } from '@storybook/react';

import { SvgIconProps } from '.';
import { ArrowLeft } from './ArrowLeft';
import { ArrowRightIcon } from './ArrowRight';
import { Book } from './Book';
import { CheckIcon } from './Check';
import { CheckboxChecked } from './CheckboxChecked';
import { CheckboxEmpty } from './CheckboxEmpty';
import { CheckboxIndeterminate } from './CheckboxIndeterminate';
import { ChevronDown } from './ChevronDown';
import { Circle } from './Circle';
import { Clock } from './Clock';
import { Close } from './Close';
import { Copy } from './Copy';
import { Customers } from './Customers';
import { Dashboard } from './Dashboard';
import { Delete } from './Delete';
import { Download } from './Download';
import { Edit } from './Edit';
import { Home } from './Home';
import { InfoIcon } from './Info';
import { InvoiceIcon } from './Invoice';
import { MSupplyGuy } from './MSupplyGuy';
import { MenuDots } from './MenuDots';
import { Messages } from './Messages';
import { PlusCircle } from './PlusCircle';
import { Power } from './Power';
import { Printer } from './Printer';
import { Radio } from './Radio';
import { Reports } from './Reports';
import { Rewind } from './Rewind';
import { Settings } from './Settings';
import { SortAsc } from './SortAsc';
import { SortDesc } from './SortDesc';
import { Stock } from './Stock';
import { Suppliers } from './Suppliers';
import { Tools } from './Tools';
import { Translate } from './Translate';
import { UserIcon } from './User';
import { XCircleIcon } from './XCircle';
import { Box, Grid, Paper, styled, TextField, Typography } from '@mui/material';

export default {
  title: 'Assets/Svg Icon',
  component: Grid,
  argTypes: {
    backgroundColor: { control: 'color' },
  },
} as ComponentMeta<typeof Grid>;

const StyledPaper = styled(Paper)({
  textAlign: 'center',
  height: 90,
  padding: 10,
  width: 150,
});
type Icon = {
  icon: JSX.Element;
  name: string;
};

const Template: ComponentStory<React.FC<SvgIconProps>> = args => {
  const icons: Icon[] = [
    { icon: <ArrowLeft {...args} />, name: 'ArrowLeft' },
    { icon: <ArrowRightIcon {...args} />, name: 'ArrowRight' },
    { icon: <Book {...args} />, name: 'Book' },
    { icon: <CheckIcon {...args} />, name: 'Check' },
    { icon: <CheckboxChecked {...args} />, name: 'CheckboxChecked' },
    {
      icon: <CheckboxIndeterminate {...args} />,
      name: 'Checkbox Indeterminate',
    },
    { icon: <CheckboxEmpty {...args} />, name: 'CheckboxEmpty' },
    { icon: <ChevronDown {...args} />, name: 'ChevronDown' },
    { icon: <Circle htmlColor="#e95c30" {...args} />, name: 'Circle' },
    { icon: <Clock {...args} />, name: 'Clock' },
    { icon: <Close {...args} />, name: 'Close' },
    { icon: <Copy {...args} />, name: 'Copy' },
    { icon: <Customers {...args} />, name: 'Customers' },
    { icon: <Dashboard {...args} />, name: 'Dashboard' },
    { icon: <Delete {...args} />, name: 'Delete' },
    { icon: <Download {...args} />, name: 'Download' },
    { icon: <Edit {...args} />, name: 'Edit' },
    { icon: <Home {...args} />, name: 'Home' },
    { icon: <InfoIcon {...args} />, name: 'Info' },
    { icon: <InvoiceIcon {...args} />, name: 'Invoice' },
    { icon: <MSupplyGuy {...args} size="medium" />, name: 'MSupplyGuy' },
    { icon: <MenuDots {...args} />, name: 'MenuDots' },
    { icon: <Messages {...args} />, name: 'Messages' },
    { icon: <PlusCircle {...args} />, name: 'PlusCircle' },
    { icon: <Power {...args} />, name: 'Power' },
    { icon: <Printer {...args} />, name: 'Printer' },
    { icon: <Radio {...args} />, name: 'Radio' },
    { icon: <Reports {...args} />, name: 'Reports' },
    { icon: <Rewind {...args} />, name: 'Rewind' },
    { icon: <Settings {...args} />, name: 'Settings' },
    { icon: <SortAsc {...args} />, name: 'SortAsc' },
    { icon: <SortDesc {...args} />, name: 'SortDesc' },
    { icon: <Stock {...args} />, name: 'Stock' },
    { icon: <Suppliers {...args} />, name: 'Suppliers' },
    { icon: <Tools {...args} />, name: 'Tools' },
    { icon: <Translate {...args} />, name: 'Translate' },
    { icon: <UserIcon {...args} />, name: 'User' },
    { icon: <XCircleIcon {...args} />, name: 'XCircle' },
  ];
  const [filteredIcons, setFilteredIcons] = useState(icons);
  const filterIcons = (event: ChangeEvent<HTMLInputElement>) => {
    const re = new RegExp(event.target.value, 'i');
    setFilteredIcons(icons.filter(i => re.test(i.name)));
  };
  return (
    <>
      <Box padding={1}>
        <TextField
          onChange={filterIcons}
          label="Filter icons"
          variant="outlined"
        />
      </Box>
      <Grid item>
        <Grid container spacing={1}>
          {filteredIcons.map(i => (
            <Grid item xs key={i.name}>
              <StyledPaper>
                {i.icon}
                <Typography>{i.name}</Typography>
              </StyledPaper>
            </Grid>
          ))}
        </Grid>
      </Grid>
    </>
  );
};

export const Primary = Template.bind({});
export const Secondary = Template.bind({});
export const Small = Template.bind({});

Secondary.args = { color: 'secondary' } as SvgIconProps;
Small.args = { fontSize: 'small' } as SvgIconProps;
