<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.019.12 Post Create Ticket Request Data</name>
   <tag></tag>
   <elementGuidId>ce00df15-fd5e-4584-8570-e0cf53997047</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;projectId\&quot;: \&quot;66e901c165b533fbe0c72c03\&quot;,\n  \&quot;moduleId\&quot;: \&quot;66e901c265b533fbe0c72c5a\&quot;,\n  \&quot;subModuleId\&quot;: \&quot;66e9399b526ab43e0426e37d\&quot;,\n  \&quot;description\&quot;: \&quot;Request data for Q3 financial analysis.\&quot;,\n  \&quot;headers\&quot;: [\n    \&quot;Name\&quot;,\n    \&quot;Email\&quot;,\n    \&quot;Department\&quot;\n  ],\n  \&quot;rows\&quot;: [\n    {\n      \&quot;name\&quot;: \&quot;John Doe\&quot;,\n      \&quot;email\&quot;: \&quot;john@example.com\&quot;,\n      \&quot;department\&quot;: \&quot;HR\&quot;\n    },\n    {\n      \&quot;name\&quot;: \&quot;Jane Smith\&quot;,\n      \&quot;email\&quot;: \&quot;jane@example.com\&quot;,\n      \&quot;department\&quot;: \&quot;Finance\&quot;\n    }\n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9499f9e7-5133-4f30-8f9f-49db7fc6724e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b0e58668-1fe9-445e-996c-ef50e6cddea5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>062f9fe7-f707-4cce-8ff5-0adb0366dd11</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ticket}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'/ticket/request-data'</defaultValue>
      <description></description>
      <id>ba3e3b48-d50b-4698-80f8-cf75dd316cfc</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>d8e48ae2-64e2-49a1-a626-809c2b59c24a</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ticket</defaultValue>
      <description></description>
      <id>052f2831-caf8-401e-a9c0-01d9481218dc</id>
      <masked>false</masked>
      <name>ticket</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
