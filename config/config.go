package config

import (
	"fmt"
	"log"

	"github.com/spf13/viper"
)

type Config struct {
	DbUser string
	DbPwd  string
	DbName string
	DbHost string
	DbPort uint
}

var AppConfig Config

func LoadConfig() {
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	viper.AddConfigPath(".")
	viper.AddConfigPath("./config")
	viper.AddConfigPath("..")

	viper.AutomaticEnv()

	viper.SetDefault("environment", "")

	environment := viper.GetString("GO_ENV")
	if environment != "" {
		fmt.Println("Using environment:", environment)
		viper.SetConfigName("config." + environment)
	}

	if err := viper.ReadInConfig(); err != nil {
		log.Fatalf("Failed read the config file: %v", err)
	}

	AppConfig.DbUser = viper.GetString("database.user")
	AppConfig.DbPwd = viper.GetString("database.password")
	AppConfig.DbName = viper.GetString("database.name")
	AppConfig.DbHost = viper.GetString("database.host")
	AppConfig.DbPort = viper.GetUint("database.port")
}

func (c *Config) GetConnectionString() string {
	return fmt.Sprintf("user=%s password=%s dbname=%s host=%s port=%d sslmode=disable",
		c.DbUser, c.DbPwd, c.DbName, c.DbHost, c.DbPort)
}
