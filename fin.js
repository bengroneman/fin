const { program } = require('commander');
const { Sequelize, Model, DataTypes } = require('sequelize');
const path = require("path");
const sequelize = new Sequelize({
    dialect: 'sqlite',
    storage: './findb.sqlite',
});
program
    .option('-a, --amount <int>')
    .option('-b, --bucket <char>');

program.parse();

const options = program.opts();

class Transaction extends Model {}

Transaction.init({
    amount: DataTypes.INTEGER,
    bucket: DataTypes.STRING,
}, { sequelize, modelName: 'Transaction'});


(async () => {
    await sequelize.sync();
    await Transaction.create({
        amount: options.amount,
        bucket: options.bucket,
    });
})();


