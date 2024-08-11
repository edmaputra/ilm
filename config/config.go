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
	DbPort uint8
}

var AppConfig Config

func LoadConfig() {
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	viper.AddConfigPath(".")
	viper.AddConfigPath("./config")

	viper.AutomaticEnv()

	if err := viper.ReadInConfig(); err != nil {
		log.Fatalf("Failed read the config file: %v", err)
	}

	AppConfig.DbUser = viper.GetString("database.user")
	AppConfig.DbPwd = viper.GetString("database.password")
	AppConfig.DbName = viper.GetString("database.name")
	AppConfig.DbHost = viper.GetString("database.host")
	AppConfig.DbPort = uint8(viper.GetUint("database.port"))
}

func (c *Config) GetConnectionString() string {
	return fmt.Sprintf("user=%s password=%s dbname=%s host=%s port=%d sslmode=disable",
		c.DbUser, c.DbPwd, c.DbName, c.DbHost, c.DbPort)
}
