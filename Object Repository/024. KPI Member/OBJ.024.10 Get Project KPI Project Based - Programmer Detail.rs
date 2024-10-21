<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.024.10 Get Project KPI Project Based - Programmer Detail</name>
   <tag></tag>
   <elementGuidId>abe1614b-3b92-4904-9a89-ea17eaf62aa8</elementGuidId>
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
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>c12dbb03-6869-4da7-b6e4-e68d2088bfec</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${kpi_member}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>384ca34d-012f-4adf-bdfe-8b9ba1d4e8bb</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'/api/project/66e901c165b533fbe0c72c03/project-based/programmer/member/66e90a38e64712c4e5eb10d1/team/FE'</defaultValue>
      <description></description>
      <id>83422b2e-36f3-43c1-ad27-8518481e337a</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.kpi_member</defaultValue>
      <description></description>
      <id>89033065-2922-4d82-a6f1-2ebd6e8a1c2f</id>
      <masked>false</masked>
      <name>kpi_member</name>
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
